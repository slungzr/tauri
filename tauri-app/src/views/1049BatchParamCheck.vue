<template>
  <div class="batch-param-check">
    <div style="margin-bottom: 16px;">
      <el-button type="primary" @click="goToBrowser">
        <el-icon><Back /></el-icon>
        返回数据浏览
      </el-button>
    </div>
    <div class="control-panel">
      <h2>1049参数选择</h2>
      
      <div class="search-panel">
        <el-select
          v-model="selectedSysInfo"
          placeholder="选择SysInfo参数"
          clearable
          class="search-input"
          @change="handleSysInfoChange"
        >
          <el-option
            v-for="option in sysInfoOptions"
            :key="option.value"
            :label="option.label"
            :value="option.value"
          />
        </el-select>
        <el-button 
          type="primary" 
          @click="refreshOptions"
          :loading="loading"
        >
          刷新选项
        </el-button>
        <el-button 
          type="success" 
          @click="startParamCheck"
          :loading="checking"
          :disabled="!selectedSysInfo"
        >
          参数检查
        </el-button>
        <el-button 
          type="info" 
          @click="exportToCSV"
          :disabled="!filteredCheckResults.length"
        >
          导出CSV
        </el-button>
      </div>

      <!-- 检查项选择 -->
      <div class="check-options-panel">
        <div v-for="category in checkCategories" :key="category.key" class="category-group">
          <div class="category-title">
            <strong>{{ category.label }}</strong>
            <el-button-group>
              <el-button type="text" @click="toggleCategory(category, true)">全选</el-button>
              <el-button type="text" @click="toggleCategory(category, false)">全不选</el-button>
            </el-button-group>
          </div>
          <el-checkbox-group v-model="selectedChecks" class="category-items">
            <el-checkbox 
              v-for="item in category.children" 
              :key="item.key" 
              :label="item.key"
              class="check-item"
            >
              {{ item.label }}
            </el-checkbox>
          </el-checkbox-group>
        </div>
      </div>

      <!-- 检查进度 -->
      <div class="progress-info" v-if="checking">
        <el-progress 
          :percentage="progressPercentage" 
          :format="progressFormat"
        />
        <div class="progress-text">
          已检查: {{ checkedCount }} / {{ totalCrossIds }}
        </div>
      </div>

      <!-- 检查结果 -->
      <div class="check-results" v-if="checkResults.length > 0">
        <el-card>
          <template #header>
            <div class="results-header">
              <div>
                <span>检查结果</span>
                <span v-if="filterCrossId || filterCrossName" class="filter-info">
                  (显示 {{ filteredCheckResults.length }} / {{ checkResults.length }} 条记录)
                </span>
              </div>
              <div class="filter-panel">
                <el-input
                  v-model="filterCrossId"
                  placeholder="按路口ID筛选"
                  clearable
                  style="width: 200px; margin-right: 10px;"
                >
                  <template #prefix>
                    <el-icon><Search /></el-icon>
                  </template>
                </el-input>
                <el-input
                  v-model="filterCrossName"
                  placeholder="按路口名称筛选"
                  clearable
                  style="width: 200px;"
                >
                  <template #prefix>
                    <el-icon><Search /></el-icon>
                  </template>
                </el-input>
              </div>
            </div>
          </template>
          <el-table
            :data="filteredCheckResults"
            style="width: 100%"
            border
            height="calc(100vh - 300px)"
          >
            <el-table-column prop="crossId" label="路口ID" width="150" fixed>
            </el-table-column>
            <el-table-column prop="crossName" label="路口名称" width="200" fixed>
            </el-table-column>
            <el-table-column prop="status" label="状态" width="100" fixed>
              <template #default="scope">
                <el-tag :type="scope.row.status === 'success' ? 'success' : 'danger'">
                  {{ scope.row.status === 'success' ? '通过' : '失败' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="errorCount" label="错误数量" width="100" fixed>
              <template #default="scope">
                <el-tag :type="scope.row.errorCount === 0 ? 'success' : 'danger'">
                  {{ scope.row.errorCount }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="errors" label="错误信息" min-width="300">
              <template #default="scope">
                <div v-if="scope.row.errors && scope.row.errors.length > 0">
                  <div 
                    v-for="(error, index) in scope.row.errors" 
                    :key="index"
                    class="error-item"
                  >
                    {{ error }}
                  </div>
                </div>
                <span v-else class="success-text">✓ 验证通过</span>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { invoke } from '@tauri-apps/api/tauri'
import { useRouter } from 'vue-router'
import { Back, Search } from '@element-plus/icons-vue'

export default {
  name: '1049BatchParamCheck',
  setup() {
    const router = useRouter()
    
    const selectedSysInfo = ref('')
    const sysInfoOptions = ref([])
    const loading = ref(false)
    const checking = ref(false)
    const checkResults = ref([])
    const checkedCount = ref(0)
    const totalCrossIds = ref(0)
    const filterCrossId = ref('')
    const filterCrossName = ref('')

    const checkCategories = ref([
      { 
        key: 'group_crossParam', 
        label: '路口参数', 
        children: [
          { key: 'crossID_check', label: '交叉口编号(crossID)匹配' },
          { key: 'crossName_check', label: '交叉口名称(crossName)存在' },
          { key: 'feature_check', label: '形状(feature)格式' },
          { key: 'latlon_check', label: '中心坐标(latitude/longitude)格式' }
        ] 
      },
      { 
        key: 'group_dayPlan', 
        label: '日计划参数', 
        children: [
          { key: 'dayPlanNo_check', label: '日计划编号(dayPlanNo)匹配' },
          { key: 'dayPlanCrossID_check', label: '日计划内交叉口编号(crossID)匹配' },
          { key: 'dayPlanName_check', label: '日计划名称(dayPlanName)存在' },
          { key: 'periodList_structure_check', label: '时段列表(periodList)结构' },
          { key: 'period_startTime_unique_check', label: '时段开始时间(startTime)唯一性' },
          { key: 'period_fields_check', label: '时段字段(startTime, ctrlMode, planNo)格式' },
        ]
      },
      {
        key: 'group_planExistence',
        label: '时段方案存在性',
        children: [
            { key: 'plan_existence_check', label: '检查非特殊控制时段控制方案是否存在' }
        ]
      },
      {
        key: 'group_laneParam',
        label: '车道参数',
        children: [
            { key: 'laneNo_check', label: '车道序号(laneNo)匹配和范围' },
            { key: 'laneCrossID_check', label: '车道内交叉口编号(crossID)匹配' },
            { key: 'laneAttribute_check', label: '车道属性(attribute)有效性' },
            { key: 'laneFeature_check', label: '车道特征(feature)有效性' },
            { key: 'laneAzimuth_check', label: '行车航向角(azimuth)格式' },
            { key: 'laneMovement_check', label: '当前转向(movement)有效性' },
        ]
      },
      {
        key: 'group_laneDistribution',
        label: '车道分布合理性',
        children: [
            { key: 'lane_import_check', label: '必须有进口车道' },
            { key: 'lane_export_check', label: '必须有出口车道' },
            { key: 'lane_motor_nonmotor_check', label: '各进口必须有机动车和非机动车道' },
            { key: 'lane_azimuth_consistency_check', label: '同方向同属性车道航向角一致性(误差<15°)' },
            { key: 'lane_import_azimuth_diff_check', label: '各进口方向车道航向角差异合理性(相差>45°)' },
            { key: 'lane_export_azimuth_diff_check', label: '各出口方向车道航向角差异合理性(相差>45°)' },
        ]
      }
    ])
    
    const allCheckKeys = checkCategories.value.flatMap(cat => cat.children.map(child => child.key))
    const selectedChecks = ref([...allCheckKeys]) // 默认全选

    // 全选/全不选分类
    const toggleCategory = (category, select) => {
      const categoryKeys = category.children.map(child => child.key)
      if (select) {
        selectedChecks.value = [...new Set([...selectedChecks.value, ...categoryKeys])]
      } else {
        selectedChecks.value = selectedChecks.value.filter(key => !categoryKeys.includes(key))
      }
    }

    const fullKey = computed(() => {
      if (selectedSysInfo.value) {
        return `1049Cache:param:SysInfo:${selectedSysInfo.value}`
      }
      return ''
    })

    const progressPercentage = computed(() => {
      if (totalCrossIds.value === 0) return 0
      return Math.round((checkedCount.value / totalCrossIds.value) * 100)
    })

    const progressFormat = (percentage) => {
      return `${checkedCount.value}/${totalCrossIds.value}`
    }

    // 获取SysInfo参数选项
    const fetchSysInfoOptions = async () => {
      loading.value = true
      try {
        const response = await invoke('search_keys', {
          pattern: '1049Cache:param:SysInfo:*'
        })

        if (response.success && response.data) {
          const keys = response.data
          const options = keys.map(key => {
            // 提取SysInfo后面的部分作为选项值
            const parts = key.split(':')
            const value = parts[parts.length - 1] // 获取最后一部分
            return {
              label: `${value} (${key})`,
              value: value
            }
          })
          sysInfoOptions.value = options
          console.log('获取到SysInfo选项:', options)
          ElMessage.success(`成功获取 ${options.length} 个SysInfo参数选项`)
        } else {
          ElMessage.error('获取SysInfo参数失败')
        }
      } catch (error) {
        console.error('获取SysInfo选项失败:', error)
        ElMessage.error(`获取SysInfo选项失败: ${error}`)
      } finally {
        loading.value = false
      }
    }

    // 处理SysInfo选择变化
    const handleSysInfoChange = (value) => {
      if (value) {
        console.log('选中的SysInfo参数:', value)
        console.log('完整Key:', fullKey.value)
      }
    }

    // 刷新选项
    const refreshOptions = () => {
      fetchSysInfoOptions()
    }

    // 开始参数检查
    const startParamCheck = async () => {
      if (!selectedSysInfo.value) {
        ElMessage.warning('请先选择SysInfo参数')
        return
      }

      checking.value = true
      checkResults.value = []
      checkedCount.value = 0

      try {
        // 1. 获取SysInfo数据
        const sysInfoResponse = await invoke('get_key_data', {
          key: fullKey.value
        })

        if (!sysInfoResponse.success) {
          throw new Error(`获取SysInfo数据失败: ${sysInfoResponse.error}`)
        }

        // 2. 解析SysInfo数据，获取路口ID列表
        let sysInfoData
        try {
          if (sysInfoResponse.data.type === 'string') {
            sysInfoData = JSON.parse(sysInfoResponse.data.value)
          } else if (sysInfoResponse.data.type === 'hash') {
            const valueStr = sysInfoResponse.data.value?.value || sysInfoResponse.data.value
            if (typeof valueStr === 'string') {
              sysInfoData = JSON.parse(valueStr)
            } else {
              sysInfoData = sysInfoResponse.data.value
            }
          }
        } catch (parseError) {
          throw new Error(`解析SysInfo数据失败: ${parseError.message}`)
        }

        if (!sysInfoData || !sysInfoData.crossIDList || !sysInfoData.crossIDList.crossID) {
          throw new Error('SysInfo数据格式错误，缺少crossIDList.crossID字段')
        }

        const crossIds = sysInfoData.crossIDList.crossID
        totalCrossIds.value = crossIds.length

        if (crossIds.length === 0) {
          ElMessage.warning('未找到路口ID')
          return
        }

        console.log(`找到 ${crossIds.length} 个路口ID:`, crossIds)

        // 3. 逐个检查每个路口的参数，全部检查完毕后再渲染
        const tempResults = []
        
        // 使用并发处理，但限制并发数量以避免过载
        const concurrencyLimit = 5 // 同时处理5个路口
        const chunks = []
        for (let i = 0; i < crossIds.length; i += concurrencyLimit) {
          chunks.push(crossIds.slice(i, i + concurrencyLimit))
        }

        for (let chunkIndex = 0; chunkIndex < chunks.length; chunkIndex++) {
          const chunk = chunks[chunkIndex]
          
          // 并发处理当前批次的路口
          const chunkPromises = chunk.map(async (crossId, indexInChunk) => {
            const globalIndex = chunkIndex * concurrencyLimit + indexInChunk
            checkedCount.value = globalIndex + 1
            
            try {
              const result = await checkCrossParams(crossId, selectedChecks.value)
              return result
            } catch (error) {
              return {
                crossId: crossId,
                crossName: '未知',
                status: 'error',
                errorCount: 1,
                errors: [`检查失败: ${error.message}`]
              }
            }
          })

          const chunkResults = await Promise.all(chunkPromises)
          tempResults.push(...chunkResults)
        }
        
        checkResults.value = tempResults

        ElMessage.success(`检查完成，共检查 ${crossIds.length} 个路口`)
      } catch (error) {
        console.error('参数检查失败:', error)
        ElMessage.error(`参数检查失败: ${error}`)
      } finally {
        checking.value = false
      }
    }

    // 检查单个路口的参数
    const checkCrossParams = async (crossId, selectedChecks) => {
      const errors = []
      let crossName = '未知'

      // 车道参数验证字典
      const LaneMovementDict = {
        '11': '直行',
        '12': '左转',
        '13': '右转',
        '21': '直左混行',
        '22': '直右混行',
        '23': '左右混行',
        '24': '直左右混行',
        '31': '掉头',
        '32': '掉头加左转',
        '33': '掉头加直行',
        '34': '掉头加右转',
        '99': '其他'
      }

      const LaneAttributeDict = {
        '0': '路口进口',
        '1': '路口出口',
        '2': '匝道',
        '3': '路段车道',
        '9': '其他'
      }

      const LaneFeatureDict = {
        '1': '机动车道',
        '2': '非机动车道',
        '3': '机非混合车道',
        '4': '人行便道',
        '9': '其他'
      }

      try {
        // 1. 获取CrossParam数据
        const crossParamKey = `1049Cache:param:CrossParam:${crossId}`
        const crossParamResponse = await invoke('get_key_data', {
          key: crossParamKey
        })

        if (!crossParamResponse.success) {
          errors.push(`获取CrossParam失败: ${crossParamResponse.error}`)
          return {
            crossId: crossId,
            crossName: crossName,
            status: 'error',
            errorCount: errors.length,
            errors: errors
          }
        }

        // 2. 解析CrossParam数据
        let crossParamData
        try {
          if (crossParamResponse.data.type === 'string') {
            crossParamData = JSON.parse(crossParamResponse.data.value)
          } else if (crossParamResponse.data.type === 'hash') {
            const valueStr = crossParamResponse.data.value?.value || crossParamResponse.data.value
            if (typeof valueStr === 'string') {
              crossParamData = JSON.parse(valueStr)
            } else {
              crossParamData = crossParamResponse.data.value
            }
          }
        } catch (parseError) {
          errors.push(`解析CrossParam数据失败: ${parseError.message}`)
          return {
            crossId: crossId,
            crossName: crossName,
            status: 'error',
            errorCount: errors.length,
            errors: errors
          }
        }

        if (!crossParamData) {
          errors.push('CrossParam数据为空')
          return {
            crossId: crossId,
            crossName: crossName,
            status: 'error',
            errorCount: errors.length,
            errors: errors
          }
        }

        console.log(`路口 ${crossId} 的CrossParam数据:`, crossParamData)

        // 3. 检查指定的路口参数
        if (selectedChecks.includes('crossID_check')) {
          if (!crossParamData.crossID) {
            errors.push('缺少交叉口编号(crossID)')
          } else if (crossParamData.crossID !== crossId) {
            errors.push(`交叉口编号不匹配，期望: ${crossId}，实际: ${crossParamData.crossID}`)
          }
        }

        // 3.2 检查交叉口名称
        if (selectedChecks.includes('crossName_check')) {
          if (!crossParamData.crossName) {
            errors.push('缺少交叉口名称(crossName)')
          } else {
            crossName = crossParamData.crossName
          }
        }

        // 3.3 检查形状（非必选）
        if (selectedChecks.includes('feature_check')) {
          if (crossParamData.feature !== undefined) {
            if (typeof crossParamData.feature !== 'number') {
              errors.push(`形状(feature)格式错误，应为数字，当前值: ${crossParamData.feature}`)
            }
          }
        }

        // 3.4 检查交叉口中心位置坐标
        if (selectedChecks.includes('latlon_check')) {
          if (!crossParamData.latitude || !crossParamData.longitude) {
            errors.push('缺少交叉口中心位置坐标(latitude/longitude)')
          } else {
            if (typeof crossParamData.latitude !== 'number' || 
                crossParamData.latitude < -90 || crossParamData.latitude > 90) {
              errors.push(`纬度(latitude)格式错误，应为-90到90之间的数字，当前值: ${crossParamData.latitude}`)
            }
            if (typeof crossParamData.longitude !== 'number' || 
                crossParamData.longitude < -180 || crossParamData.longitude > 180) {
              errors.push(`经度(longitude)格式错误，应为-180到180之间的数字，当前值: ${crossParamData.longitude}`)
            }
          }
        }
        
        // 3.5 检查日计划列表
        if (crossParamData.dayPlanNoList && crossParamData.dayPlanNoList.dayPlanNo) {
          const dayPlanNos = crossParamData.dayPlanNoList.dayPlanNo
          console.log(`路口 ${crossId} 有 ${dayPlanNos.length} 个日计划:`, dayPlanNos)

          // 并发检查所有日计划
          const dayPlanPromises = dayPlanNos.map(async (dayPlanNo) => {
            try {
              const dayPlanParamKey = `1049Cache:param:DayPlanParam:${crossId}:${dayPlanNo}`
              const dayPlanParamResponse = await invoke('get_key_data', { key: dayPlanParamKey })
              if (!dayPlanParamResponse.success) return [`获取日计划${dayPlanNo}参数失败: ${dayPlanParamResponse.error}`]

              let dayPlanParamData
              try {
                if (dayPlanParamResponse.data.type === 'string') dayPlanParamData = JSON.parse(dayPlanParamResponse.data.value)
                else if (dayPlanParamResponse.data.type === 'hash') {
                  const valueStr = dayPlanParamResponse.data.value?.value || dayPlanParamResponse.data.value
                  if (typeof valueStr === 'string') dayPlanParamData = JSON.parse(valueStr)
                  else dayPlanParamData = dayPlanParamResponse.data.value
                }
              } catch (parseError) { return [`解析日计划${dayPlanNo}参数失败: ${parseError.message}`] }

              if (!dayPlanParamData) return [`日计划${dayPlanNo}参数数据为空`]
              console.log(`日计划${dayPlanNo}参数:`, dayPlanParamData)
              
              const currentDayPlanErrors = []
              const validatedErrors = validateDayPlanParams(dayPlanParamData, dayPlanNo, crossId, selectedChecks)
              currentDayPlanErrors.push(...validatedErrors)
              
              // 检查时段对应的PlanParam是否存在
              if (selectedChecks.includes('plan_existence_check')) {
                if (dayPlanParamData.periodList && dayPlanParamData.periodList.period) {
                  const periods = dayPlanParamData.periodList.period
                  const periodPromises = periods.map(async (period) => {
                    if (period.ctrlMode !== '11' && period.ctrlMode !== '12' && period.planNo !== undefined && period.planNo !== null) {
                      const planParamKey = `1049Cache:param:PlanParam:${crossId}:${period.planNo}`
                      const planParamResponse = await invoke('get_key_data', { key: planParamKey })
                      if (!planParamResponse.success) return `日计划${dayPlanNo}时段${period.startTime}对应的方案${period.planNo}不存在`
                    }
                    return null
                  })
                  const periodResults = await Promise.all(periodPromises)
                  currentDayPlanErrors.push(...periodResults.filter(Boolean))
                }
              }

              return currentDayPlanErrors
            } catch (error) { return [`检查日计划${dayPlanNo}参数失败: ${error.message}`] }
          })

          const dayPlanResults = await Promise.all(dayPlanPromises)
          dayPlanResults.forEach(dayPlanErrors => { errors.push(...dayPlanErrors) })
        } else { console.log(`路口 ${crossId} 没有日计划列表信息`) }
        
        // 4. 获取车道列表并进行验证
        if (crossParamData.laneNoList && crossParamData.laneNoList.laneNo) {
          const laneNos = crossParamData.laneNoList.laneNo
          console.log(`路口 ${crossId} 有 ${laneNos.length} 个车道:`, laneNos)

          const laneAttributes = { '0': 0, '1': 0, '2': 0, '3': 0, '9': 0 }
          const directionFeatures = {}

          // 5. 并发获取每个车道的LaneParam数据并进行验证
          const lanePromises = laneNos.map(async (laneNo) => {
            try {
              const laneParamKey = `1049Cache:param:LaneParam:${crossId}:${laneNo}`
              const laneParamResponse = await invoke('get_key_data', { key: laneParamKey })
              if (!laneParamResponse.success) return { errors: [`获取车道${laneNo}参数失败: ${laneParamResponse.error}`], laneParamData: null }

              let laneParamData
              try {
                if (laneParamResponse.data.type === 'string') laneParamData = JSON.parse(laneParamResponse.data.value)
                else if (laneParamResponse.data.type === 'hash') {
                  const valueStr = laneParamResponse.data.value?.value || laneParamResponse.data.value
                  if (typeof valueStr === 'string') laneParamData = JSON.parse(valueStr)
                  else laneParamData = laneParamResponse.data.value
                }
              } catch (parseError) { return { errors: [`解析车道${laneNo}参数失败: ${parseError.message}`], laneParamData: null } }

              if (!laneParamData) return { errors: [`车道${laneNo}参数数据为空`], laneParamData: null }

              const laneErrors = validateLaneParams(laneParamData, laneNo, crossId, selectedChecks, LaneMovementDict, LaneAttributeDict, LaneFeatureDict)
              return { errors: laneErrors, laneParamData: laneParamData }
            } catch (error) { return { errors: [`检查车道${laneNo}参数失败: ${error.message}`], laneParamData: null } }
          })

          const laneResults = await Promise.all(lanePromises)
          
          laneResults.forEach(result => {
            errors.push(...result.errors)
            if (result.laneParamData) {
              const { attribute, direction, feature } = result.laneParamData
              if (attribute !== undefined && attribute !== null) {
                const attrKey = attribute.toString()
                if (laneAttributes.hasOwnProperty(attrKey)) laneAttributes[attrKey]++
              }
              if (attribute === 0 && direction !== undefined && feature !== undefined) {
                const dirStr = direction.toString(), featStr = feature.toString()
                if (!directionFeatures[dirStr]) directionFeatures[dirStr] = { '1': 0, '2': 0, '3': 0, '4': 0, '9': 0 }
                if (directionFeatures[dirStr].hasOwnProperty(featStr)) directionFeatures[dirStr][featStr]++
              }
            }
          })

          // 6. 验证车道属性分布
          if (selectedChecks.includes('lane_import_check')) {
            if (laneAttributes['0'] === 0) errors.push('缺少车道属性为路口进口的车道')
          }
          if (selectedChecks.includes('lane_export_check')) {
            if (laneAttributes['1'] === 0) errors.push('缺少车道属性为路口出口的车道')
          }
          if (selectedChecks.includes('lane_motor_nonmotor_check')) {
            Object.keys(directionFeatures).forEach(direction => {
              const features = directionFeatures[direction]
              const motorLanes = features['1']
              const nonMotorLanes = features['2'] + features['3']
              if (motorLanes === 0) errors.push(`进口${direction}缺少机动车道`)
              if (nonMotorLanes === 0) errors.push(`进口${direction}缺少非机动车道或机非混合车道`)
            })
          }
          if (selectedChecks.includes('lane_azimuth_consistency_check')) {
            const groupedAzimuths = {}
            const groupedLaneNos = {}
            laneResults.forEach(result => {
              if (result.laneParamData) {
                const { direction, attribute, azimuth, laneNo } = result.laneParamData
                if (direction !== undefined && attribute !== undefined && azimuth !== undefined) {
                  const groupKey = `${direction}_${attribute}`
                  if (!groupedAzimuths[groupKey]) groupedAzimuths[groupKey] = []
                  groupedAzimuths[groupKey].push(azimuth)
                  if (!groupedLaneNos[groupKey]) groupedLaneNos[groupKey] = []
                  groupedLaneNos[groupKey].push(laneNo)
                }
              }
            })
            Object.keys(groupedAzimuths).forEach(key => {
              const azimuths = groupedAzimuths[key]
              const laneNos = groupedLaneNos[key] || []
              if (azimuths.length > 1) {
                const minAzimuth = Math.min(...azimuths)
                const maxAzimuth = Math.max(...azimuths)
                if (maxAzimuth - minAzimuth > 15) {
                  const [direction, attribute] = key.split('_')
                  const attributeLabel = LaneAttributeDict[attribute] || '未知属性'
                  errors.push(`方向${direction} ${attributeLabel}的车道航向角差异过大(>15°)，车道号: [${laneNos.join(', ')}]，角度: [${azimuths.join(', ')}]`)
                }
              }
            })
          }
          // 新增：各进口方向车道航向角差异合理性校验
          if (selectedChecks.includes('lane_import_azimuth_diff_check')) {
            // 收集所有进口车道（attribute=0），按direction分组
            const importAzimuthsByDirection = {}
            laneResults.forEach(result => {
              if (result.laneParamData) {
                const { direction, attribute, azimuth } = result.laneParamData
                if (attribute === 0 && direction !== undefined && azimuth !== undefined) {
                  const dirKey = direction.toString()
                  if (!importAzimuthsByDirection[dirKey]) importAzimuthsByDirection[dirKey] = []
                  importAzimuthsByDirection[dirKey].push(azimuth)
                }
              }
            })
            // 计算每个方向的平均航向角
            const dirAvgAzimuth = {}
            Object.keys(importAzimuthsByDirection).forEach(dir => {
              const azimuths = importAzimuthsByDirection[dir]
              if (azimuths.length > 0) {
                const avg = azimuths.reduce((a, b) => a + b, 0) / azimuths.length
                dirAvgAzimuth[dir] = avg
              }
            })
            // 对所有方向两两组合，计算平均航向角差值
            const dirs = Object.keys(dirAvgAzimuth)
            for (let i = 0; i < dirs.length; i++) {
              for (let j = i + 1; j < dirs.length; j++) {
                const dirA = dirs[i]
                const dirB = dirs[j]
                const azA = dirAvgAzimuth[dirA]
                const azB = dirAvgAzimuth[dirB]
                // 计算最小角度差（考虑360°环绕）
                let diff = Math.abs(azA - azB)
                if (diff > 180) diff = 360 - diff
                if (diff <= 45) {
                  errors.push(`进口方向${dirA}与${dirB}的平均航向角差值为${diff.toFixed(1)}°，小于等于45°，不合理（${azA.toFixed(1)}° vs ${azB.toFixed(1)}°）`)
                }
              }
            }
          }
          // 新增：各出口方向车道航向角差异合理性校验
          if (selectedChecks.includes('lane_export_azimuth_diff_check')) {
            // 收集所有出口车道（attribute=1），按direction分组
            const exportAzimuthsByDirection = {}
            laneResults.forEach(result => {
              if (result.laneParamData) {
                const { direction, attribute, azimuth } = result.laneParamData
                if (attribute === 1 && direction !== undefined && azimuth !== undefined) {
                  const dirKey = direction.toString()
                  if (!exportAzimuthsByDirection[dirKey]) exportAzimuthsByDirection[dirKey] = []
                  exportAzimuthsByDirection[dirKey].push(azimuth)
                }
              }
            })
            // 计算每个方向的平均航向角
            const dirAvgAzimuth = {}
            Object.keys(exportAzimuthsByDirection).forEach(dir => {
              const azimuths = exportAzimuthsByDirection[dir]
              if (azimuths.length > 0) {
                const avg = azimuths.reduce((a, b) => a + b, 0) / azimuths.length
                dirAvgAzimuth[dir] = avg
              }
            })
            // 对所有方向两两组合，计算平均航向角差值
            const dirs = Object.keys(dirAvgAzimuth)
            for (let i = 0; i < dirs.length; i++) {
              for (let j = i + 1; j < dirs.length; j++) {
                const dirA = dirs[i]
                const dirB = dirs[j]
                const azA = dirAvgAzimuth[dirA]
                const azB = dirAvgAzimuth[dirB]
                // 计算最小角度差（考虑360°环绕）
                let diff = Math.abs(azA - azB)
                if (diff > 180) diff = 360 - diff
                if (diff <= 45) {
                  errors.push(`出口方向${dirA}与${dirB}的平均航向角差值为${diff.toFixed(1)}°，小于等于45°，不合理（${azA.toFixed(1)}° vs ${azB.toFixed(1)}°）`)
                }
              }
            }
          }
        } else { errors.push('CrossParam缺少车道列表信息') }

      } catch (error) { errors.push(`检查路口参数失败: ${error.message}`) }

      return {
        crossId: crossId,
        crossName: crossName,
        status: errors.length === 0 ? 'success' : 'error',
        errorCount: errors.length,
        errors: errors
      }
    }

    // 验证车道参数
    const validateLaneParams = (data, laneNo, crossId, selectedChecks, LaneMovementDict, LaneAttributeDict, LaneFeatureDict) => {
      const errors = []

      if (selectedChecks.includes('laneNo_check')) {
        if (!data.laneNo) errors.push(`车道${laneNo}: 缺少车道序号(laneNo)`)
        else if (data.laneNo !== laneNo) errors.push(`车道${laneNo}: 车道序号不匹配，期望: ${laneNo}，实际: ${data.laneNo}`)
        else if (data.laneNo < 1 || data.laneNo > 99) errors.push(`车道${laneNo}: 车道序号超出范围，应为1~99，当前值: ${data.laneNo}`)
      }
      if (selectedChecks.includes('laneCrossID_check')) {
        if (!data.crossID) errors.push(`车道${laneNo}: 缺少交叉口编号(crossID)`)
        else if (data.crossID !== crossId) errors.push(`车道${laneNo}: 交叉口编号不匹配，期望: ${crossId}，实际: ${data.crossID}`)
      }
      if (selectedChecks.includes('laneAttribute_check')) {
        if (data.attribute === undefined || data.attribute === null) errors.push(`车道${laneNo}: 缺少车道属性(attribute)`)
        else if (!Object.keys(LaneAttributeDict).includes(data.attribute.toString())) errors.push(`车道${laneNo}: 车道属性值无效，当前值：${data.attribute}`)
      }
      if (selectedChecks.includes('laneFeature_check')) {
        if (data.feature === undefined || data.feature === null) errors.push(`车道${laneNo}: 缺少车道特征(feature)`)
        else if (!Object.keys(LaneFeatureDict).includes(data.feature.toString())) errors.push(`车道${laneNo}: 车道特征值无效，当前值：${data.feature}`)
      }
      if (selectedChecks.includes('laneAzimuth_check')) {
        if (data.azimuth === undefined || data.azimuth === null) errors.push(`车道${laneNo}: 缺少车道行车航向角(azimuth)`)
        else if (typeof data.azimuth !== 'number' || data.azimuth < 0 || data.azimuth > 359) errors.push(`车道${laneNo}: 车道行车航向角格式错误，应为0~359的整数，当前值: ${data.azimuth}`)
      }
      if (selectedChecks.includes('laneMovement_check')) {
        if (data.movement === undefined || data.movement === null) errors.push(`车道${laneNo}: 缺少当前转向(movement)`)
        else if (!Object.keys(LaneMovementDict).includes(data.movement.toString())) errors.push(`车道${laneNo}: 当前转向值无效，当前值：${data.movement}`)
      }
      return errors
    }

    // 验证日计划参数
    const validateDayPlanParams = (data, dayPlanNo, crossId, selectedChecks) => {
      const errors = []

      if (selectedChecks.includes('dayPlanNo_check')) {
        if (!data.dayPlanNo) errors.push(`日计划${dayPlanNo}: 缺少日计划编号(dayPlanNo)`)
        else if (data.dayPlanNo !== dayPlanNo) errors.push(`日计划${dayPlanNo}: 日计划编号不匹配，期望: ${dayPlanNo}，实际: ${data.dayPlanNo}`)
        else if (data.dayPlanNo < 1 || data.dayPlanNo > 99) errors.push(`日计划${dayPlanNo}: 日计划编号超出范围，应为1~99`)
      }
      if (selectedChecks.includes('dayPlanCrossID_check')) {
        if (!data.crossID) errors.push(`日计划${dayPlanNo}: 缺少交叉口编号(crossID)`)
        else if (data.crossID !== crossId) errors.push(`日计划${dayPlanNo}: 交叉口编号不匹配，期望: ${crossId}，实际: ${data.crossID}`)
      }
      if (selectedChecks.includes('dayPlanName_check')) {
        if (!data.dayPlanName) errors.push(`日计划${dayPlanNo}: 缺少日计划名称(dayPlanName)`)
      }
      if (selectedChecks.includes('periodList_structure_check')) {
        if (!data.periodList) errors.push(`日计划${dayPlanNo}: 缺少时段列表(periodList)`)
        else if (!data.periodList.period) errors.push(`日计划${dayPlanNo}: 缺少时段数组(periodList.period)`)
        else if (!Array.isArray(data.periodList.period)) errors.push(`日计划${dayPlanNo}: 时段数组格式错误`)
        else if (data.periodList.period.length === 0) errors.push(`日计划${dayPlanNo}: 时段数组不能为空`)
      }
      
      if (data.periodList && Array.isArray(data.periodList.period)) {
        if (selectedChecks.includes('period_startTime_unique_check')) {
          const startTimes = data.periodList.period.map(p => p.startTime)
          if (new Set(startTimes).size !== startTimes.length) {
            const duplicates = startTimes.filter((t, i) => startTimes.indexOf(t) !== i)
            errors.push(`日计划${dayPlanNo}: 时段开始时间重复: ${[...new Set(duplicates)].join(', ')}`)
          }
        }
        if (selectedChecks.includes('period_fields_check')) {
          data.periodList.period.forEach((period, index) => {
            if (!period.startTime) errors.push(`日计划${dayPlanNo}: 第${index + 1}个时段缺少开始时间(startTime)`)
            else if (!/^\d{2}:\d{2}$/.test(period.startTime)) errors.push(`日计划${dayPlanNo}: 第${index + 1}个时段开始时间格式错误(HH:MM)`)
            if (period.ctrlMode === undefined) errors.push(`日计划${dayPlanNo}: 第${index + 1}个时段缺少控制模式(ctrlMode)`)
            if (period.planNo === undefined) errors.push(`日计划${dayPlanNo}: 第${index + 1}个时段缺少方案编号(planNo)`)
          })
        }
      }
      return errors
    }

    // 导出CSV
    const exportToCSV = () => {
      if (filteredCheckResults.value.length === 0) {
        ElMessage.warning('没有检查结果可导出')
        return
      }

      const csvContent = generateCSV()
      const blob = new Blob(['\ufeff' + csvContent], { type: 'text/csv;charset=utf-8;' })
      const link = document.createElement('a')
      const url = URL.createObjectURL(blob)
      link.setAttribute('href', url)
      link.setAttribute('download', `1049参数检查结果_${new Date().toISOString().slice(0, 19).replace(/:/g, '-')}.csv`)
      link.style.visibility = 'hidden'
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      
      ElMessage.success('CSV文件导出成功')
    }

    // 生成CSV内容
    const generateCSV = () => {
      const headers = ['路口ID', '路口名称', '状态', '错误数量', '错误信息']
      const rows = filteredCheckResults.value.map(result => [
        result.crossId,
        result.crossName,
        result.status === 'success' ? '通过' : '失败',
        result.errorCount,
        result.errors ? result.errors.join('; ') : ''
      ])

      return [headers, ...rows]
        .map(row => row.map(cell => `"${cell}"`).join(','))
        .join('\n')
    }

    // 返回到Browser页面
    const goToBrowser = () => {
      router.push({ name: 'Browser' })
    }

    // 过滤后的检查结果
    const filteredCheckResults = computed(() => {
      if (!filterCrossId.value && !filterCrossName.value) {
        return checkResults.value
      }
      
      return checkResults.value.filter(result => {
        const crossIdMatch = !filterCrossId.value || 
          result.crossId.toLowerCase().includes(filterCrossId.value.toLowerCase())
        const crossNameMatch = !filterCrossName.value || 
          result.crossName.toLowerCase().includes(filterCrossName.value.toLowerCase())
        return crossIdMatch && crossNameMatch
      })
    })

    // 页面加载时获取SysInfo选项
    fetchSysInfoOptions()

    return {
      selectedSysInfo,
      sysInfoOptions,
      loading,
      checking,
      checkResults,
      checkedCount,
      totalCrossIds,
      fullKey,
      progressPercentage,
      progressFormat,
      checkCategories,
      selectedChecks,
      toggleCategory,
      handleSysInfoChange,
      refreshOptions,
      startParamCheck,
      exportToCSV,
      goToBrowser,
      filterCrossId,
      filterCrossName,
      filteredCheckResults
    }
  }
}
</script>

<style scoped>
.batch-param-check {
  padding: 20px;
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.control-panel {
  margin-bottom: 20px;
}

.control-panel h2 {
  margin-bottom: 20px;
  color: #303133;
}

.search-panel {
  display: flex;
  gap: 10px;
  align-items: center;
  margin-bottom: 20px;
}

.check-options-panel {
  margin-bottom: 20px;
  padding: 10px;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
}

.category-group {
  margin-bottom: 10px;
}

.category-group:last-child {
  margin-bottom: 0;
}

.category-title {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 5px;
}

.category-items {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.check-item {
  width: calc(33.33% - 10px);
  margin-right: 0 !important; /*覆盖element-plus默认样式*/
}

.search-input {
  width: 300px;
}

.result-panel {
  margin-top: 20px;
}

.selected-info p {
  margin: 10px 0;
  font-size: 14px;
}

.selected-info strong {
  color: #409eff;
}

.progress-info {
  margin-top: 20px;
}

.progress-text {
  margin-top: 10px;
  text-align: center;
  color: #606266;
}

.check-results {
  margin-top: 20px;
}

.error-item {
  color: #f56c6c;
  margin-bottom: 5px;
  font-size: 12px;
}

.success-text {
  color: #67c23a;
  font-size: 12px;
}

.el-button + .el-button {
  margin-left: 8px;
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.filter-panel {
  display: flex;
  align-items: center;
}

.filter-info {
  color: #909399;
  font-size: 14px;
  margin-left: 8px;
}
</style>
