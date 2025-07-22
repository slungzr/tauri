<template>
  <div class="gb-param-batch-check">
    <el-container>
      <el-header class="header">
        <h2>GB参数批量检查</h2>
        <el-button 
          type="primary" 
          @click="goToSingleCheck"
        >
          返回单路口检查
        </el-button>
      </el-header>
      <el-main>
        <div class="control-panel">
          <el-input
            v-model="searchPattern"
            placeholder="输入key前缀，例如: V2X:1001:* 或 V2X:1001:*|正则表达式，V2X:1001:*|320271[0-9]{4}$"
            class="search-input"
          >
            <template #prepend>Key前缀</template>
          </el-input>
          <el-button 
            type="primary" 
            @click="startBatchCheck"
            :loading="checking"
          >
            开始批量检查
          </el-button>
          <el-button 
            type="success" 
            @click="exportToCSV"
            :disabled="validationResults.length === 0"
          >
            导出CSV
          </el-button>
          <el-button 
            type="info" 
            @click="exportPassedToCSV"
            :disabled="!validationResults.some(r => r.status === 'success')"
          >
            导出通过列表
          </el-button>
        </div>

        <div v-if="validationResults.length > 0" class="results-container">
          <div class="search-panel">
            <el-input
              v-model="crossNameFilter"
              placeholder="请输入路口名称"
              class="search-input"
              clearable
            >
              <template #prepend>路口名称</template>
            </el-input>
            <el-button 
              type="primary" 
              @click="handleSearch"
            >
              搜索
            </el-button>
          </div>

          <el-table
            :data="filteredResults"
            style="width: 100%"
            border
            height="calc(100vh - 320px)"
            :row-height="getRowHeight"
          >
            <el-table-column prop="crossId" label="路口ID" width="120" fixed />
            <el-table-column prop="crossName" label="路口名称" width="150" fixed />
            <el-table-column prop="status" label="检查状态" width="100" fixed>
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
          
            <el-table-column prop="errors" label="错误详情" min-width="400">
              <template #default="scope">
                <div v-if="scope.row.errors.length > 0" class="error-list">
                  <div v-for="(error, index) in scope.row.errors" :key="index" class="error-item">
                    {{ error }}
                  </div>
                </div>
                <span v-else>-</span>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="180" fixed="right">
              <template #default="scope">
                <el-button
                  type="primary"
                  size="small"
                  @click="viewDetails(scope.row.crossId)"
                >
                  查看详情
                </el-button>
                <el-button
                  type="success"
                  size="small"
                  @click="goToSingleCheck(scope.row.crossId)"
                >
                  单路口检查
                </el-button>
              </template>
            </el-table-column>
          </el-table>
        </div>

        <div v-else-if="checking" class="loading-container">
          <el-progress 
            :percentage="progressPercentage" 
            :format="progressFormat"
          />
          <div class="loading-text">正在检查中，请稍候...</div>
        </div>

        <div v-else class="empty-container">
          <el-empty description="暂无检查结果" />
        </div>
      </el-main>
    </el-container>
    <el-dialog
      v-model="jsonDialogVisible"
      title="数据详情"
      width="80%"
      :destroy-on-close="true"
      class="json-dialog"
    >
      <el-tabs v-model="activeTab" type="border-card">
        <el-tab-pane label="路口基本信息" name="1001">
          <div class="tab-content">
            <div class="error-list" v-if="jsonData['1001']?.errors?.length">
              <h4>错误信息：</h4>
              <div v-for="(error, index) in jsonData['1001'].errors" :key="index" class="error-item">
                {{ error }}
              </div>
            </div>
            <div class="json-viewer">
              <vue-json-pretty
                v-if="jsonData['1001']?.json"
                :data="jsonData['1001'].json"
                :deep="999"
                :show-double-quotes="true"
                :show-length="true"
                :collapsed-strings-length="50"
                :error-fields="getErrorFields('1001')"
                @click="handleJsonClick"
              />
            </div>
          </div>
        </el-tab-pane>
        
        <el-tab-pane label="信号配时方案" name="2001">
          <div class="tab-content">
            <div class="error-list" v-if="jsonData['2001']?.errors?.length">
              <h4>错误信息：</h4>
              <div v-for="(error, index) in jsonData['2001'].errors" :key="index" class="error-item">
                {{ error }}
              </div>
            </div>
            <div class="json-viewer">
              <vue-json-pretty
                v-if="jsonData['2001']?.json"
                :data="jsonData['2001'].json"
                :deep="999"
                :show-double-quotes="true"
                :show-length="true"
                :collapsed-strings-length="50"
                :error-fields="getErrorFields('2001')"
                @click="handleJsonClick"
              />
            </div>
          </div>
        </el-tab-pane>
        
        <el-tab-pane label="信号周期开始数据" name="2002">
          <div class="tab-content">
            <div class="error-list" v-if="jsonData['2002']?.errors?.length">
              <h4>错误信息：</h4>
              <div v-for="(error, index) in jsonData['2002'].errors" :key="index" class="error-item">
                {{ error }}
              </div>
            </div>
            <div class="json-viewer">
              <vue-json-pretty
                v-if="jsonData['2002']?.json"
                :data="jsonData['2002'].json"
                :deep="999"
                :show-double-quotes="true"
                :show-length="true"
                :collapsed-strings-length="50"
                :error-fields="getErrorFields('2002')"
                @click="handleJsonClick"
              />
            </div>
          </div>
        </el-tab-pane>
        
        <el-tab-pane label="灯组灯色状态" name="2003">
          <div class="tab-content">
            <div class="error-list" v-if="jsonData['2003']?.errors?.length">
              <h4>错误信息：</h4>
              <div v-for="(error, index) in jsonData['2003'].errors" :key="index" class="error-item">
                {{ error }}
              </div>
            </div>
            <div class="json-viewer">
              <vue-json-pretty
                v-if="jsonData['2003']?.json"
                :data="jsonData['2003'].json"
                :deep="999"
                :show-double-quotes="true"
                :show-length="true"
                :collapsed-strings-length="50"
                :error-fields="getErrorFields('2003')"
                @click="handleJsonClick"
              />
            </div>
          </div>
        </el-tab-pane>
      </el-tabs>
    </el-dialog>
  </div>
</template>

<script>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/tauri'
import { ElMessage } from 'element-plus'
import { save } from '@tauri-apps/api/dialog'
import VueJsonPretty from 'vue-json-pretty'
import 'vue-json-pretty/lib/styles.css'

// 车道属性定义
const LaneAttributeDict = {
  '0': '路口进口',
  '1': '路口出口',
  '2': '匝道',
  '3': '路段车道',
  '9': '其他'
}

// 车道特征定义
const LaneFeatureDict = {
  '1': '机动车道',
  '2': '非机动车道',
  '3': '机非混合车道',
  '4': '人行便道',
  '9': '其他'
}

export default {
  name: 'GBParamBatchCheck',
  components: {
    VueJsonPretty
  },
  setup() {
    const router = useRouter()
    const searchPattern = ref('V2X:1001:*')
    const checking = ref(false)
    const validationResults = ref([])
    const totalCrossIds = ref(0)
    const checkedCount = ref(0)
    const tableHeight = ref(600)
    const jsonDialogVisible = ref(false)
    const jsonData = ref({})
    const activeTab = ref('1001')
    const crossNameFilter = ref('')
    const filteredResults = ref([])

    const progressPercentage = computed(() => {
      if (totalCrossIds.value === 0) return 0
      return Math.round((checkedCount.value / totalCrossIds.value) * 100)
    })

    const progressFormat = (percentage) => {
      return `${checkedCount.value}/${totalCrossIds.value}`
    }

    const goToSingleCheck = async (crossId) => {
      if (!crossId) {
        router.push({
          name: 'GBParamCheck'
        })
        return
      }

      router.push({
        name: 'GBParamCheck',
        query: { crossId }
      })
    }

    const viewDetails = async (crossId) => {
      try {
        // 获取所有相关数据
        const dataTypes = ['1001', '2001', '2002', '2003']
        const results = {}

        for (const type of dataTypes) {
          const key = `V2X:${type}:${crossId}`
          const response = await invoke('get_key_data', { key })
          
          if (response.success) {
            const json = JSON.parse(response.data.value)
            let errors = []
            
            // 根据数据类型调用相应的验证函数
            switch (type) {
              case '1001':
                errors = await validateCrossData(json)
                break
              case '2001':
                errors = await validateSignalData(json)
                break
              case '2002':
                errors = await validateCycleData(json)
                break
              case '2003':
                errors = await validateLampStatusData(json)
                break 
            }
            
            results[type] = { json, errors }
          }
        }

        jsonData.value = results
        jsonDialogVisible.value = true
      } catch (error) {
        ElMessage.error(`获取数据失败: ${error.message}`)
      }
    }

    const validateCrossData = async (data) => {
      const errors = []
      
      // 检查必填字段
      const requiredFields = {
        recordTime: '数据更新时间',
        crossName: '交叉口名称',
        crossId: '交叉口编号',
        position: '交叉口中心位置坐标',
        lanes: '车道对象集合',
        peds: '人行横道对象集合'
      }

      for (const [field, label] of Object.entries(requiredFields)) {
        if (!data[field]) {
          errors.push(`[1001]${label}(${field})为必填项`)
        }
      }

      // 验证recordTime格式
      if (data.recordTime && !/^\d{14}$/.test(data.recordTime)) {
        errors.push(`[1001]数据更新时间(recordTime)格式错误，应为YYYYMMDDhhmmss格式`)
      }

      // 验证position格式
      if (data.position && !/^-?\d+(\.\d+)?,-?\d+(\.\d+)?$/.test(data.position)) {
        errors.push(`[1001]交叉口中心位置坐标(position)格式错误，应为经度,纬度格式`)
      }

      // 验证车道数据
      if (data.lanes && Array.isArray(data.lanes)) {
        // 检查是否缺少必要类型的车道
        const laneTypes = data.lanes.map(lane => lane.type)
        const missingTypes = []
        
        if (!laneTypes.includes(1)) {
          missingTypes.push('机动车车道')
        }
        
        // 检查是否有非机动车道或机非混合车道
        const hasNonMotorLane = laneTypes.some(type => type === 2 || type === 3)
        if (!hasNonMotorLane) {
          missingTypes.push('非机动车道或机非混合车道')
        }

        // 检查是否有出口道
        const hasExitLane = data.lanes.some(lane => lane.attr == '1')
        if (!hasExitLane) {
          missingTypes.push('机动车出口道')
        }

        if (missingTypes.length > 0) {
          errors.push(`[1001]缺少以下类型的车道：${missingTypes.join('、')}`)
        }

        // 按方向+属性分组检查车道航向角
        /* 用于按方向+属性分组存储车道 */
        const directionAttrGroups = new Map()

        // 检查每个车道的详细信息
        data.lanes.forEach((lane, index) => {
          console.log(lane)
          // 检查车道属性
          if (lane.attr === undefined || lane.attr === null || lane.attr === '') {
            
            errors.push(`[1001]第${index + 1}条车道的属性(attr)为必填项`)
          } else if (!Object.keys(LaneAttributeDict).includes(lane.attr.toString())) {
            errors.push(`[1001]第${index + 1}条车道的属性(attr)值无效，当前值：${lane.attr}，有效值：${Object.keys(LaneAttributeDict).join('、')}`)
          }

          // 检查车道类型
          if (lane.type === undefined || lane.type === null || lane.type === '') {
            errors.push(`[1001]第${index + 1}条车道的类型(type)为必填项`)
          } else if (!Object.keys(LaneFeatureDict).includes(lane.type.toString())) {
            errors.push(`[1001]第${index + 1}条车道的类型(type)值无效，当前值：${lane.type}，有效值：${Object.keys(LaneFeatureDict).join('、')}`)
          }

          // 检查车道航向角
          if (lane.angle === undefined || lane.angle === null || lane.angle === '') {
            errors.push(`[1001]第${index + 1}条车道的行车航向角(angle)为必填项`)
          } else {
            const angle = Number(lane.angle)
            if (isNaN(angle)) {
              errors.push(`[1001]第${index + 1}条车道的行车航向角(angle)必须为数字，当前值：${lane.angle}`)
            } else if (angle < 0 || angle > 360) {
              errors.push(`[1001]第${index + 1}条车道的行车航向角(angle)必须在0-360度之间，当前值：${angle}`)
            }

            // 将车道按方向+属性分组（只对非匝道和非路段车道的车道进行分组）
            if (lane.direction !== undefined && lane.direction !== null && lane.direction !== '' && 
                lane.attr !== '2' && lane.attr !== '3') {
              const groupKey = `${lane.direction}_${lane.attr}`
              if (!directionAttrGroups.has(groupKey)) {
                directionAttrGroups.set(groupKey, [])
              }
              directionAttrGroups.get(groupKey).push({
                index: index + 1,
                angle: angle,
                attr: lane.attr
              })
            }
          }

          // 检查可变车道的计划变换信息
          // if (lane.isVar === 1) {
          //   if (!lane.changeTime) {
          //     errors.push(`[1001]第${index + 1}条可变车道缺少计划变换时间(changeTime)`)
          //   }
          //   if (!lane.nextDirection) {
          //     errors.push(`[1001]第${index + 1}条可变车道缺少计划变换转向(nextDirection)`)
          //   }
          // }

          // 检查待行区信息
          // if (lane.waitingArea === 1) {
          //   if (!lane.waitingAreaPosition) {
          //     errors.push(`[1001]第${index + 1}条车道的待行区缺少中心线位置点坐标(waitingAreaPosition)`)
          //   } else if (!/^-?\d+(\.\d+)?,-?\d+(\.\d+)?$/.test(lane.waitingAreaPosition)) {
          //     errors.push(`[1001]第${index + 1}条车道的待行区中心线位置点坐标(waitingAreaPosition)格式错误，应为经度,纬度格式`)
          //   }
          // }
        })

        // 检查每个方向+属性下的车道航向角是否相同
        directionAttrGroups.forEach((lanes, groupKey) => {
          const angles = new Set(lanes.map(lane => lane.angle))
          if (angles.size > 1) {
            const laneIndices = lanes.map(lane => lane.index).join('、')
            const [direction, attr] = groupKey.split('_')
            errors.push(`[1001]方向${direction}属性${attr}下的车道（第${laneIndices}条）航向角不一致，存在${angles.size}个不同的值：${Array.from(angles).join('、')}`)
          }
        })
      }

      // 验证人行横道数据
      if (data.peds && Array.isArray(data.peds)) {
        if (data.peds.length === 0) {
          errors.push(`[1001]缺少人行横道数据`)
        } else {
          // data.peds.forEach((ped, index) => {
          //   // 检查人行横道位置坐标（中心线起点和终点，GCJ-02坐标系，经纬度用逗号分隔，多个点用分号分隔）
          //   if (!ped.positions) {
          //     errors.push(`[1001]第${index + 1}条人行横道缺少位置坐标(positions)`)
          //   } else {
          //     // 检查格式：经度,纬度;经度,纬度
          //     const posArr = ped.positions.split(';')
          //     if (posArr.length < 2) {
          //       errors.push(`[1001]第${index + 1}条人行横道位置坐标(positions)格式错误，应至少包含起点和终点（经度,纬度;经度,纬度）`)
          //     } else {
          //       for (let i = 0; i < posArr.length; i++) {
          //         if (!/^-?\d+(\.\d+)?,-?\d+(\.\d+)?$/.test(posArr[i].trim())) {
          //           errors.push(`[1001]第${index + 1}条人行横道位置坐标(positions)第${i + 1}个点格式错误，应为经度,纬度格式`)
          //         }
          //       }
          //     }
          //   }
          // })
        }
      }

      return errors
    }

    const validateSignalData = async (data) => {
      const errors = []
      
      // 检查必填字段
      const requiredFields = {
        recordTime: '数据更新时间',
        crossId: '交叉口编号',
        crossName: '交叉口名称',
        position: '交叉口中心位置坐标',
        curPlanStartTime: '当前方案时段开始时间',
        curPlan: '当前方案配时信息对象'
      }

      for (const [field, label] of Object.entries(requiredFields)) {
        if (!data[field]) {
          errors.push(`[2001]${label}(${field})为必填项`)
        }
      }

      // 验证recordTime格式
      if (data.recordTime && !/^\d{14}$/.test(data.recordTime)) {
        errors.push(`[2001]数据更新时间(recordTime)格式错误，应为YYYYMMDDhhmmss格式，当前值：${data.recordTime}`)
      }

      // 验证curPlanStartTime格式
      if (data.curPlanStartTime && !/^\d{6}$/.test(data.curPlanStartTime)) {
        errors.push(`[2001]当前方案时段开始时间(curPlanStartTime)格式错误，应为HHmmss格式，当前值：${data.curPlanStartTime}`)
      }

      // 验证nextPlanStartTime格式（如果存在）
      if (data.nextPlanStartTime && !/^\d{6}$/.test(data.nextPlanStartTime)) {
        errors.push(`[2001]下一方案时段开始时间(nextPlanStartTime)格式错误，应为HHmmss格式，当前值：${data.nextPlanStartTime}`)
      }

      // 验证时段是否匹配当前时间
      if (data.recordTime && data.curPlanStartTime) {
        const recordTime = new Date(
          data.recordTime.slice(0, 4),
          parseInt(data.recordTime.slice(4, 6)) - 1,
          data.recordTime.slice(6, 8),
          data.recordTime.slice(8, 10),
          data.recordTime.slice(10, 12),
          data.recordTime.slice(12, 14)
        )
        
        // 解析当前方案开始时间
        const curPlanHour = parseInt(data.curPlanStartTime.slice(0, 2))
        const curPlanMinute = parseInt(data.curPlanStartTime.slice(2, 4))
        const curPlanSecond = parseInt(data.curPlanStartTime.slice(4, 6))
        const curPlanTime = new Date(
          recordTime.getFullYear(),
          recordTime.getMonth(),
          recordTime.getDate(),
          curPlanHour,
          curPlanMinute,
          curPlanSecond
        )

        // 解析下一方案开始时间（如果存在）
        let nextPlanTime = null
        if (data.nextPlanStartTime) {
          const nextPlanHour = parseInt(data.nextPlanStartTime.slice(0, 2))
          const nextPlanMinute = parseInt(data.nextPlanStartTime.slice(2, 4))
          const nextPlanSecond = parseInt(data.nextPlanStartTime.slice(4, 6))
          nextPlanTime = new Date(
            recordTime.getFullYear(),
            recordTime.getMonth(),
            recordTime.getDate(),
            nextPlanHour,
            nextPlanMinute,
            nextPlanSecond
          )
        }

        // 检查当前时间是否在时段内
        if (recordTime < curPlanTime) {
          errors.push(`[2001]当前时间(${data.recordTime})早于方案开始时间(${data.curPlanStartTime})`)
        }
        if (nextPlanTime && recordTime >= nextPlanTime) {
          errors.push(`[2001]当前时间(${data.recordTime})晚于或等于下一方案开始时间(${data.nextPlanStartTime})`)
        }
      }

      // 验证色步表
      if (data.curPlan && data.curPlan.lampColorStepPlanList) {
        const plans = data.curPlan.lampColorStepPlanList
        if (!Array.isArray(plans) || plans.length === 0) {
          errors.push(`[2001]灯组配时方案列表(lampColorStepPlanList)必须为非空数组`)
        } else {
          // 计算每个灯组的总时长
          const groupTotalTimes = new Map()
          
          plans.forEach((plan, planIndex) => {
            if (!plan.colorStepList || !Array.isArray(plan.colorStepList)) {
              errors.push(`[2001]第${planIndex + 1}个灯组配时方案缺少色步列表(colorStepList)或格式错误`)
              return
            }

            const groupId = `${plan.controlDir}-${plan.lampType}`
            let totalTime = 0

            plan.colorStepList.forEach((step, stepIndex) => {
              if (!step.duration || !step.stepNo || !step.lampStatus) {
                errors.push(`[2001]第${planIndex + 1}个灯组配时方案的第${stepIndex + 1}个色步缺少必要字段`)
                return
              }

              const duration = Number(step.duration)
              if (isNaN(duration) || duration <= 0) {
                errors.push(`[2001]第${planIndex + 1}个灯组配时方案的第${stepIndex + 1}个色步时长(duration)必须为正数，当前值：${step.duration}`)
                return
              }

              totalTime += duration
            })

            if (!groupTotalTimes.has(groupId)) {
              groupTotalTimes.set(groupId, totalTime)
            } else if (Math.abs(groupTotalTimes.get(groupId) - totalTime) > 0.001) {
              errors.push(`[2001]灯组${groupId}的总时长(${totalTime}秒)与其他灯组不一致，第一个灯组总时长为${groupTotalTimes.get(groupId)}秒`)
            }
          })
        }
      }

      return errors
    }

    const validateLampStatusData = async (data) => {
      const errors = []
      
      // 检查必填字段
      const requiredFields = {
        recordTime: '数据更新时间',
        crossId: '交叉口编号',
        timestamp: '时间戳',
        lampStatusList: '灯组灯色状态列表'
      }

      for (const [field, label] of Object.entries(requiredFields)) {
        if (data[field] === undefined || data[field] === null || data[field] === '') {
          errors.push(`[2003]信号灯组灯色状态数据中${label}(${field})为必填项，当前值：${data[field]}`)
        }
      }

      // 验证recordTime格式
      if (data.recordTime && !/^\d{14}(\d{3})?$/.test(data.recordTime)) {
        errors.push(`[2003]信号灯组灯色状态数据更新时间(recordTime)格式错误，应为YYYYMMDDhhmmss或YYYYMMDDhhmmssSSS格式，当前值：${data.recordTime}`)
      }

      // 验证timestamp是否为有效的时间戳
      if (data.timestamp !== undefined && (isNaN(data.timestamp) || data.timestamp < 0)) {
        errors.push(`[2003]时间戳(timestamp)必须为非负整数，当前值：${data.timestamp}`)
      }

      // 验证lampStatusList
      if (data.lampStatusList && Array.isArray(data.lampStatusList)) {
        // 获取当前方案类型
        let planType = null
        try {
          const signalDataKey = `V2X:2001:${data.crossId}`
          const signalDataResponse = await invoke('get_key_data', { key: signalDataKey })
          if (signalDataResponse.success) {
            const signalData = JSON.parse(signalDataResponse.data.value)
            if (signalData.curPlan && signalData.curPlan.planType !== undefined) {
              planType = signalData.curPlan.planType
            }
          }
        } catch (error) {
          console.error('获取方案类型失败:', error)
        }

        data.lampStatusList.forEach(async (lamp, index) => {
          // 检查灯组必填字段
          const lampRequiredFields = {
            controlDir: '指示的进口方向',
            lampType: '灯组类型',
            lampStatus: '灯组灯色状态',
            remainTime: '信号灯组灯色状态剩余时长'
          }

          for (const [field, label] of Object.entries(lampRequiredFields)) {
            if (lamp[field] === undefined || lamp[field] === null || lamp[field] === '') {
              errors.push(`[2003]第${index + 1}个灯组的${label}(${field})为必填项，当前值：${lamp[field]}`)
            }
          }

          // 验证controlDir范围（航向角）
          if (lamp.controlDir !== undefined && (lamp.controlDir < 0 || lamp.controlDir > 360)) {
            errors.push(`[2003]第${index + 1}个灯组的进口方向(controlDir)必须在0-360度之间，当前值：${lamp.controlDir}`)
          }

          // 验证lampType值
          const validLampTypes = ['10', '21', '22', '23', '30', '32', '40', '41', '42', '50', '61', '62', '63', '70', '80', '99']
          if (lamp.lampType !== undefined && !validLampTypes.includes(lamp.lampType.toString())) {
            errors.push(`[2003]第${index + 1}个灯组的灯组类型(lampType)值无效，当前值：${lamp.lampType}，有效值：${validLampTypes.join(', ')}`)
          }

          // 验证lampStatus值
          const validLampStatus = ['11', '21', '22', '23', '42', '43']
          if (lamp.lampStatus !== undefined && !validLampStatus.includes(lamp.lampStatus.toString())) {
            errors.push(`[2003]第${index + 1}个灯组的灯色状态(lampStatus)值无效，当前值：${lamp.lampStatus}，有效值：${validLampStatus.join(', ')}`)
          }

          // 验证remainTime范围
          if (lamp.remainTime !== undefined) {
            const remainTime = Number(lamp.remainTime)
            if (isNaN(remainTime) || remainTime < 0 || remainTime > 999) {
              errors.push(`[2003]第${index + 1}个灯组的剩余时长(remainTime)必须在0-999秒之间，当前值：${lamp.remainTime}`)
            } else if (planType === 1 && remainTime === 0) {
              errors.push(`[2003]第${index + 1}个灯组的剩余时长(remainTime)在固定配时方案下不能为0，当前值：${lamp.remainTime}`)
            }
          }
        })
      } else {
        errors.push(`[2003]灯组灯色状态列表(lampStatusList)必须为数组，当前值：${typeof data.lampStatusList}`)
      }

      return errors
    }

    const validateCycleData = async (data) => {
      const errors = []
      
      // 检查必填字段
      const requiredFields = {
        recordTime: '数据更新时间',
        crossId: '交叉口编号',
        curCycleStartTime: '当前周期开始时间',
        lastCycleLen: '上周期运行时长',
        adjustFlag: '当前周期过渡调整标记'
      }

      for (const [field, label] of Object.entries(requiredFields)) {
        if (data[field] === undefined || data[field] === null || data[field] === '') {
          errors.push(`[2002]信号周期开始数据中${label}(${field})为必填项，当前值：${data[field]}`)
        }
      }

      // 验证recordTime格式
      if (data.recordTime && !/^\d{14}$/.test(data.recordTime)) {
        errors.push(`[2002]信号周期开始数据更新时间(recordTime)格式错误，应为YYYYMMDDhhmmss格式，当前值：${data.recordTime}`)
      }

      // 验证curCycleStartTime格式
      if (data.curCycleStartTime && !/^\d{14}$/.test(data.curCycleStartTime)) {
        errors.push(`[2002]当前周期开始时间(curCycleStartTime)格式错误，应为YYYYMMDDhhmmss格式，当前值：${data.curCycleStartTime}`)
      }

      // 验证lastCycleLen范围
      if (data.lastCycleLen !== undefined && (data.lastCycleLen < 0 || data.lastCycleLen > 500)) {
        errors.push(`[2002]上周期运行时长(lastCycleLen)必须在0-500秒之间，当前值：${data.lastCycleLen}`)
      }

      // 验证adjustFlag值
      // if (data.adjustFlag === undefined || data.adjustFlag === null || data.adjustFlag === '') {
      //   errors.push(`[2002]当前周期过渡调整标记(adjustFlag)为必填项，当前值：${data.adjustFlag}`)
      // } else if (![0, 1].includes(data.adjustFlag)) {
      //   errors.push(`[2002]当前周期过渡调整标记(adjustFlag)必须为0或1，当前值：${data.adjustFlag}`)
      // }

      return errors
    }

    const startBatchCheck = async () => {
      checking.value = true
      validationResults.value = []
      checkedCount.value = 0
      const tempResults = [] // 临时存储结果

      try {
        // 处理搜索模式
        let pattern = searchPattern.value.trim()
        let crossIdRegex = null
        
        // 检查是否包含正则表达式部分
        if (pattern.includes('|')) {
          // 格式: V2X:1001:*|正则表达式
          const parts = pattern.split('|')
          if (parts.length === 2) {
            pattern = parts[0].trim()
            const regexStr = parts[1].trim()
            try {
              crossIdRegex = new RegExp(regexStr)
              console.log(`使用正则表达式过滤: ${regexStr}`)
            } catch (error) {
              throw new Error(`正则表达式格式错误: ${error.message}`)
            }
          } else {
            throw new Error('正则表达式格式错误: 请使用 "key前缀|正则表达式" 格式')
          }
        }
        
        // 处理key前缀模式
        if (pattern === 'V2X:1001:*') {
          pattern = 'V2X:1001:*'
        } else {
          if (!pattern.startsWith('V2X:1001:')) {
            pattern = `V2X:1001:${pattern}`
          }
          if (!pattern.endsWith('*')) {
            pattern = `${pattern}*`
          }
        }

        const response = await invoke('search_keys', { pattern })
        
        if (!response.success) {
          throw new Error(response.error)
        }

        // 提取路口ID并应用过滤
        let crossIds = response.data
          .map(key => key.split(':')[2])
          .filter(crossId => {
            if (crossId.length < 6) return false
            // 过滤掉第5、6位为0的路口ID
            if (crossId[4] === '0' && crossId[5] === '0') return false
            // 如果设置了正则表达式，进行匹配
            if (crossIdRegex) {
              return crossIdRegex.test(crossId)
            }
            return true
          })

        totalCrossIds.value = crossIds.length

        if (crossIds.length === 0) {
          ElMessage.warning('未找到匹配的路口ID')
          return
        }

        console.log(`找到 ${crossIds.length} 个匹配的路口ID`)
        if (crossIdRegex) {
          ElMessage.info(`使用正则表达式 "${crossIdRegex.source}" 过滤，找到 ${crossIds.length} 个路口`)
        }

        // 定义并发检查函数
        const checkCrossId = async (crossId) => {
          const errors = []
          let crossData = null
          
          try {
            // 并行获取所有数据
            const [crossDataResponse, signalDataResponse, lampStatusResponse, cycleDataResponse] = await Promise.all([
              invoke('get_key_data', { key: `V2X:1001:${crossId}` }),
              invoke('get_key_data', { key: `V2X:2001:${crossId}` }),
              invoke('get_key_data', { key: `V2X:2003:${crossId}` }),
              invoke('get_key_data', { key: `V2X:2002:${crossId}` })
            ])

            // 处理路口基本信息
            if (crossDataResponse.success) {
              try {
                crossData = JSON.parse(crossDataResponse.data.value)
                errors.push(...await validateCrossData(crossData))
              } catch (parseError) {
                errors.push(`[1001]路口基本信息数据格式错误: ${parseError.message}`)
              }
            } else {
              errors.push(`[1001]获取路口基本信息失败: ${crossDataResponse.error || '未知错误'}`)
            }

            // 处理信号配时方案
            if (signalDataResponse.success) {
              try {
                const signalData = JSON.parse(signalDataResponse.data.value)
                errors.push(...await validateSignalData(signalData))
              } catch (parseError) {
                errors.push(`[2001]信号配时方案数据格式错误: ${parseError.message}`)
              }
            } else {
              errors.push(`[2001]获取信号配时方案失败: ${signalDataResponse.error || '未知错误'}`)
            }

            // 处理灯组灯色状态
            if (lampStatusResponse.success) {
              try {
                const lampStatusData = JSON.parse(lampStatusResponse.data.value)
                errors.push(...await validateLampStatusData(lampStatusData))
              } catch (parseError) {
                errors.push(`[2003]灯组灯色状态数据格式错误: ${parseError.message}`)
              }
            } else {
              errors.push(`[2003]获取灯组灯色状态失败: ${lampStatusResponse.error || '未知错误'}`)
            }

            // 处理信号周期开始数据
            if (cycleDataResponse.success) {
              try {
                const cycleData = JSON.parse(cycleDataResponse.data.value)
                errors.push(...await validateCycleData(cycleData))
              } catch (parseError) {
                errors.push(`[2002]信号周期开始数据格式错误: ${parseError.message}`)
              }
            } else {
              errors.push(`[2002]获取信号周期开始数据失败: ${cycleDataResponse.error || '未知错误'}`)
            }

          } catch (error) {
            errors.push(`检查过程发生错误: ${error.message}`)
          }

          return {
            crossId,
            crossName: crossData?.crossName || '未知',
            status: errors.length === 0 ? 'success' : 'error',
            errorCount: errors.length,
            hasNonMotorLane: crossData?.lanes?.some(lane => lane.type === 2 || lane.type === 3) || false,
            errors
          }
        }

        // 分批处理，每批10个路口
        const batchSize = 10
        for (let i = 0; i < crossIds.length; i += batchSize) {
          const batch = crossIds.slice(i, i + batchSize)
          const batchResults = await Promise.all(batch.map(checkCrossId))
          
          // 更新临时结果和进度
          tempResults.push(...batchResults)
          checkedCount.value += batch.length
        }

        // 所有检查完成后，按路口ID排序并更新结果
        validationResults.value = tempResults.sort((a, b) => a.crossId.localeCompare(b.crossId))
        filteredResults.value = validationResults.value
      } catch (error) {
        ElMessage.error(`批量检查失败: ${error.message}`)
      } finally {
        checking.value = false
      }
    }

    const exportToCSV = async () => {
      try {
        // 准备CSV数据
        const headers = ['路口ID', '路口名称', '检查状态', '错误数量', '非机动车道', '错误详情']
        const rows = validationResults.value.map(result => [
          result.crossId,
          result.crossName,
          result.status === 'success' ? '通过' : '失败',
          result.errorCount,
          result.hasNonMotorLane ? '有' : '无',
          result.errors.join('\n')
        ])

        // 将数据转换为CSV格式
        const csvContent = [
          headers.join(','),
          ...rows.map(row => row.map(cell => {
            // 处理包含逗号、换行符或双引号的单元格
            if (typeof cell === 'string' && (cell.includes(',') || cell.includes('\n') || cell.includes('"'))) {
              return `"${cell.replace(/"/g, '""')}"`
            }
            return cell
          }).join(','))
        ].join('\n')

        // 生成文件名（使用当前时间）
        const now = new Date()
        const fileName = `路口检查结果_${now.getFullYear()}${String(now.getMonth() + 1).padStart(2, '0')}${String(now.getDate()).padStart(2, '0')}_${String(now.getHours()).padStart(2, '0')}${String(now.getMinutes()).padStart(2, '0')}.csv`

        // 打开保存对话框
        const filePath = await save({
          filters: [{
            name: 'CSV文件',
            extensions: ['csv']
          }],
          defaultPath: fileName
        })

        if (filePath) {
          // 使用main.rs中定义的write_text_file命令
          const response = await invoke('write_text_file', { 
            filePath, 
            content: csvContent 
          })
          
          if (response.success) {
            ElMessage.success('导出成功')
          } else {
            ElMessage.error(`导出失败: ${response.error || '未知错误'}`)
          }
        }
      } catch (error) {
        ElMessage.error(`导出失败: ${error.message}`)
      }
    }

    const exportPassedToCSV = async () => {
      try {
        // 只导出通过的路口
        const passed = validationResults.value.filter(result => result.status === 'success')
        if (passed.length === 0) {
          ElMessage.warning('没有通过的路口可导出')
          return
        }
        const headers = ['路口ID', '路口名称', '检查状态', '错误数量', '非机动车道']
        const rows = passed.map(result => [
          result.crossId,
          result.crossName,
          '通过',
          result.errorCount,
          result.hasNonMotorLane ? '有' : '无'
        ])
        const csvContent = [
          headers.join(','),
          ...rows.map(row => row.map(cell => {
            if (typeof cell === 'string' && (cell.includes(',') || cell.includes('\n') || cell.includes('"'))) {
              return `"${cell.replace(/"/g, '""')}"`
            }
            return cell
          }).join(','))
        ].join('\n')
        const now = new Date()
        const fileName = `通过路口列表_${now.getFullYear()}${String(now.getMonth() + 1).padStart(2, '0')}${String(now.getDate()).padStart(2, '0')}_${String(now.getHours()).padStart(2, '0')}${String(now.getMinutes()).padStart(2, '0')}.csv`
        const filePath = await save({
          filters: [{
            name: 'CSV文件',
            extensions: ['csv']
          }],
          defaultPath: fileName
        })
        if (filePath) {
          const response = await invoke('write_text_file', { 
            filePath, 
            content: csvContent 
          })
          if (response.success) {
            ElMessage.success('导出成功')
          } else {
            ElMessage.error(`导出失败: ${response.error || '未知错误'}`)
          }
        }
      } catch (error) {
        ElMessage.error(`导出失败: ${error.message}`)
      }
    }

    const getRowHeight = (row) => {
      // 根据错误数量动态计算行高
      const baseHeight = 40 // 基础行高
      const errorHeight = 20 // 每个错误项的高度
      const maxErrors = 5 // 最大显示的错误数量
      const errorCount = Math.min(row.errors.length, maxErrors)
      return baseHeight + (errorCount * errorHeight)
    }

    // 监听窗口大小变化
    const updateTableHeight = () => {
      const container = document.querySelector('.results-container')
      if (container) {
        const containerHeight = container.clientHeight
        tableHeight.value = containerHeight
      }
    }

    // 在组件挂载后和窗口大小变化时更新表格高度
    onMounted(() => {
      updateTableHeight()
      window.addEventListener('resize', updateTableHeight)
    })

    onUnmounted(() => {
      window.removeEventListener('resize', updateTableHeight)
    })

    const getJsonTitle = (key) => {
      const titles = {
        '1001': '路口基本信息',
        '2001': '信号配时方案',
        '2002': '信号周期开始数据',
        '2003': '灯组灯色状态'
      }
      return titles[key] || key
    }

    const formatJsonWithErrors = (json, errors) => {
      if (!json) return ''
      
      // 将JSON转换为格式化的字符串
      const formattedJson = JSON.stringify(json, null, 2)
      
      // 如果没有错误，直接返回格式化的JSON
      if (!errors || errors.length === 0) {
        return formattedJson
      }

      // 将错误信息转换为正则表达式模式
      const errorPatterns = errors.map(error => {
        // 提取错误信息中的字段名
        const match = error.match(/\[(\d+)\](.*?)\((\w+)\)/)
        if (match) {
          const field = match[3]
          return {
            pattern: new RegExp(`"${field}"\\s*:`, 'g'),
            error: error
          }
        }
        return null
      }).filter(Boolean)

      // 将JSON字符串分割成行
      const lines = formattedJson.split('\n')
      
      // 为每一行添加错误标记
      return lines.map(line => {
        let highlightedLine = line
        errorPatterns.forEach(({ pattern, error }) => {
          if (pattern.test(line)) {
            highlightedLine = `<span class="error-field" title="${error}">${line}</span>`
          }
        })
        return highlightedLine
      }).join('\n')
    }

    const getErrorFields = (type) => {
      const errors = jsonData.value[type]?.errors || []
      return errors.map(error => {
        const match = error.match(/\[(\d+)\](.*?)\((\w+)\)/)
        return match ? match[3] : null
      }).filter(Boolean)
    }

    const handleJsonClick = (path, data) => {
      // 可以在这里处理点击事件，例如显示详细信息
      console.log('Clicked path:', path)
      console.log('Clicked data:', data)
    }

    // 处理搜索
    const handleSearch = () => {
      if (!crossNameFilter.value) {
        filteredResults.value = validationResults.value
        return
      }

      filteredResults.value = validationResults.value.filter(item => {
        const matchName = !crossNameFilter.value || 
          item.crossName.toLowerCase().includes(crossNameFilter.value.toLowerCase())
        return matchName
      })
    }

    return {
      searchPattern,
      checking,
      validationResults,
      progressPercentage,
      progressFormat,
      goToSingleCheck,
      startBatchCheck,
      exportToCSV,
      exportPassedToCSV,
      getRowHeight,
      tableHeight,
      jsonDialogVisible,
      jsonData,
      getJsonTitle,
      formatJsonWithErrors,
      activeTab,
      getErrorFields,
      handleJsonClick,
      crossNameFilter,
      filteredResults,
      handleSearch,
      viewDetails
    }
  }
}
</script>

<style scoped>
.gb-param-batch-check {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 20px;
  background-color: #fff;
  border-bottom: 1px solid #e6e6e6;
  flex-shrink: 0;
}

.header h2 {
  margin: 0;
}

.control-panel {
  display: flex;
  gap: 20px;
  margin-bottom: 20px;
  padding: 20px;
  background-color: #fff;
  border-radius: 4px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.1);
  flex-shrink: 0;
}

.search-input {
  flex: 1;
}

.results-container {
  flex: 1;
  background-color: #fff;
  border-radius: 4px;
  padding: 20px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.1);
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  padding: 40px;
  background-color: #fff;
  border-radius: 4px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.1);
  flex: 1;
}

.loading-text {
  color: #606266;
}

.empty-container {
  padding: 40px;
  background-color: #fff;
  border-radius: 4px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.1);
  flex: 1;
}

.error-list {
  max-height: 100px;
  overflow-y: auto;
  padding-right: 8px;
}

.error-list::-webkit-scrollbar {
  width: 6px;
}

.error-list::-webkit-scrollbar-thumb {
  background-color: #909399;
  border-radius: 3px;
}

.error-list::-webkit-scrollbar-track {
  background-color: #f5f7fa;
}

.error-item {
  color: #f56c6c;
  margin-bottom: 4px;
  font-size: 13px;
  line-height: 20px;
  white-space: normal;
  word-break: break-all;
}

.error-item:last-child {
  margin-bottom: 0;
}

.json-dialog :deep(.el-dialog__body) {
  padding: 0;
  max-height: 70vh;
  overflow-y: auto;
}

.tab-content {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.error-list {
  background-color: #fef0f0;
  border-radius: 4px;
  padding: 15px;
  margin-bottom: 20px;
}

.error-list h4 {
  margin: 0 0 10px 0;
  color: #f56c6c;
  font-size: 14px;
}

.error-item {
  color: #f56c6c;
  font-size: 13px;
  line-height: 1.5;
  margin-bottom: 8px;
}

.error-item:last-child {
  margin-bottom: 0;
}

.json-content {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-all;
  font-family: monospace;
  font-size: 14px;
  line-height: 1.5;
  color: #333;
  background-color: #f8f9fa;
  padding: 15px;
  border-radius: 4px;
}

.error-field {
  background-color: #fef0f0;
  color: #f56c6c;
  padding: 2px 0;
  border-radius: 2px;
  cursor: help;
}

.json-viewer {
  background-color: #f8f9fa;
  border-radius: 4px;
  padding: 15px;
  overflow: auto;
}

:deep(.vjs-tree) {
  font-family: monospace;
  font-size: 14px;
  line-height: 1.5;
}

:deep(.vjs-tree .vjs-value) {
  color: #333;
}

:deep(.vjs-tree .vjs-key) {
  color: #881391;
}

:deep(.vjs-tree .vjs-value.vjs-value-string) {
  color: #c41a16;
}

:deep(.vjs-tree .vjs-value.vjs-value-number) {
  color: #1c00cf;
}

:deep(.vjs-tree .vjs-value.vjs-value-boolean) {
  color: #0000ff;
}

:deep(.vjs-tree .vjs-value.vjs-value-null) {
  color: #808080;
}

:deep(.vjs-tree .vjs-value.vjs-value-error) {
  background-color: #fef0f0;
  color: #f56c6c;
  padding: 2px 4px;
  border-radius: 2px;
  cursor: help;
}

:deep(.vjs-tree .vjs-value.vjs-value-error:hover) {
  background-color: #fde2e2;
}

:deep(.vjs-tree .vjs-value.vjs-value-error::after) {
  content: "!";
  margin-left: 4px;
  color: #f56c6c;
  font-weight: bold;
}

.column-header {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.filter-input {
  width: 100%;
}

.filter-input :deep(.el-input__wrapper) {
  padding: 0 8px;
}

.filter-input :deep(.el-input__inner) {
  height: 24px;
  line-height: 24px;
}

.search-panel {
  display: flex;
  gap: 16px;
  margin-bottom: 16px;
  padding: 16px;
  background-color: #fff;
  border-radius: 4px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.1);
}

.search-panel .search-input {
  width: 200px;
}

.search-panel .el-button {
  margin-left: 8px;
}

.el-button + .el-button {
  margin-left: 8px;
}
</style> 