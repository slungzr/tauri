<template>
  <div class="gb-param-check">
    <el-container>
      <el-aside width="300px" class="aside">
        <div class="aside-header">
          <h3>CrossId列表</h3>
          <div class="header-buttons">
            <el-button 
              type="primary" 
              size="small" 
              @click="goToBatchCheck"
              class="batch-check-btn"
            >
              批量检查
            </el-button>
            <el-button 
              type="primary" 
              size="small" 
              @click="goToLogin"
              class="login-btn"
            >
              返回登录
            </el-button>
          </div>
        </div>
        <div class="search-box">
          <el-input
            v-model="searchKeyword"
            placeholder="搜索路口ID"
            prefix-icon="el-icon-search"
            clearable
            @input="handleSearch"
          />
        </div>
        <el-scrollbar class="cross-list">
          <div
            v-for="item in filteredCrossIds"
            :key="item.crossId"
            :class="['cross-item', { active: activeCrossId === item.crossId }]"
            @click="handleSelect(item.crossId)"
          >
            {{ item.crossId }}
          </div>
        </el-scrollbar>
      </el-aside>
      <el-main>
        <h2>GB参数检查</h2>
        <div v-if="validationErrors.length > 0" class="validation-errors">
          <el-alert
            v-for="(error, index) in validationErrors"
            :key="index"
            :title="error"
            type="error"
            :closable="true"
            show-icon
            @close="removeError(index)"
          />
        </div>
        <div v-if="loading" class="loading">
          加载中...
        </div>
        <div v-else-if="!keys.length" class="no-data">
          暂无数据
        </div>
        <div v-else class="content">
          <div v-if="activeCrossId && crossData" class="cross-detail">
            <!-- 基本信息 -->
            <el-card class="info-card">
              <template #header>
                <div class="card-header">
                  <span>路口基本信息（1001）</span>
                  <el-button 
                    type="primary" 
                    size="small" 
                    @click="showOriginalJson('crossData')"
                  >
                    查看原始JSON
                  </el-button>
                </div>
              </template>
              <div class="cross-info">
                <el-descriptions :column="2" border>
                  <el-descriptions-item label="路口名称">{{ crossData.crossName }}</el-descriptions-item>
                  <el-descriptions-item label="路口编号">{{ crossData.crossId }}</el-descriptions-item>
                  <el-descriptions-item label="更新时间">{{ crossData.recordTime }}</el-descriptions-item>
                  <el-descriptions-item label="位置坐标">{{ crossData.position }}</el-descriptions-item>
                  <el-descriptions-item label="形状特征">{{ crossData.feature }}</el-descriptions-item>
                </el-descriptions>

                <div class="info-tables-container">
                  <!-- 车道信息表 -->
                  <div class="lane-info-table">
                    <div class="table-header">车道信息</div>
                    <el-table 
                      :data="crossData.lanes" 
                      border 
                      style="width: 100%"
                      :max-height="500"
                    >
                      <el-table-column prop="laneNo" label="车道序号" width="80" fixed>
                        <template #default="scope">
                          {{ scope.row.laneNo }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="direction" label="车道方向" width="80" fixed>
                        <template #default="scope">
                          {{ getDirectionText(scope.row.direction) }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="angle" label="行车航向角" width="100" fixed>
                        <template #default="scope">
                          {{ scope.row.angle }}°
                        </template>
                      </el-table-column>
                      <el-table-column prop="attr" label="车道属性" width="100" >
                        <template #default="scope">
                          {{ getAttributeText(scope.row.attr) }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="type" label="车道特征" width="120">
                        <template #default="scope">
                          {{ getFeatureText(scope.row.type) }}
                        </template>
                      </el-table-column>
                    
                     
                      <el-table-column prop="turn" label="当前转向" width="100">
                        <template #default="scope">
                          {{ getMovementText(scope.row.turn) }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="serviceStatus" label="服务状态" width="100">
                        <template #default="scope">
                          {{ scope.row.serviceStatus === 1 ? '正常开放' : '异常封闭' }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="isVar" label="是否可变" width="80">
                        <template #default="scope">
                          {{ scope.row.isVar === 1 ? '是' : '否' }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="changeTime" label="计划变换时间" width="120">
                        <template #default="scope">
                          {{ scope.row.changeTime || '-' }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="nextDirection" label="计划变换转向" width="120">
                        <template #default="scope">
                          {{ scope.row.nextDirection ? getDirectionText(scope.row.nextDirection) : '-' }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="waitingArea" label="有无待行区" width="100">
                        <template #default="scope">
                          {{ scope.row.waitingArea === 1 ? '有' : '无' }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="width" label="车道宽度(cm)" width="120">
                        <template #default="scope">
                          {{ scope.row.width || '-' }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="length" label="导向车道长度(cm)" width="140">
                        <template #default="scope">
                          {{ scope.row.length === 99999 ? '>99998' : (scope.row.length || '-') }}
                        </template>
                      </el-table-column>
                    </el-table>
                  </div>

                  <!-- 人行横道信息表 -->
                  <div class="ped-info-table">
                    <div class="table-header">人行横道信息</div>
                    <el-table :data="crossData.peds" border style="width: 100%">
                      <el-table-column prop="pedNo" label="序号" width="80" />
                      <el-table-column prop="direction" label="方向" width="80" />
                      <el-table-column prop="attr" label="属性" width="120">
                        <template #default="scope">
                          {{ getPedAttrText(scope.row.attr) }}
                        </template>
                      </el-table-column>
                    </el-table>
                  </div>
                </div>

                <div class="cross-map-container">
                  <div class="cross-map">
                    <CrossMap :crossData="crossData" />
                  </div>
                  <div class="lamp-status-table">
                    <div class="table-header">当前灯组灯色状态（2003） <el-button 
                    type="primary" 
                    size="small" 
                    @click="showOriginalJson('lampStatusData')"
                  >
                    查看原始JSON
                  </el-button></div>
                    <el-table :data="crossData?.lampStatusList || []" border style="width: 100%">
                      <el-table-column prop="controlDir" label="控制方向" width="120">
                        <template #default="scope">
                          {{ scope.row.controlDir }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="lampType" label="灯组类型" width="200">
                        <template #default="scope">
                          {{ getLampGroupText(scope.row.lampType) }}
                        </template>
                      </el-table-column>
                      <el-table-column prop="lampStatus" label="灯色状态" width="100">
                        <template #default="scope">
                          <div class="lamp-status" :class="{
                            'red': scope.row.lampStatus == '21',
                            'yellow': scope.row.lampStatus == '22',
                            'green': scope.row.lampStatus == '23',
                            'gray': scope.row.lampStatus == '11',
                            'yellow-flash': scope.row.lampStatus == '42',
                            'green-flash': scope.row.lampStatus == '43'
                          }">  {{ getLampStatusText(scope.row.lampStatus) }}</div>
                        </template>
                      </el-table-column>
                      <el-table-column prop="remainTime" label="剩余时间" width="120">
                        <template #default="scope">
                          {{ scope.row.remainTime }}s
                        </template>
                      </el-table-column>
                    </el-table>
                     <!-- 信号周期信息 -->
              <el-descriptions v-if="cycleData" :column="1" border class="cycle-info" title="信号周期开始(2002)">
                <el-descriptions-item label="数据更新时间">{{ cycleData.recordTime }}</el-descriptions-item>
                <el-descriptions-item label="交叉口编号">{{ cycleData.crossId }}</el-descriptions-item>
                <el-descriptions-item label="当前周期开始时间">{{ cycleData.curCycleStartTime }}</el-descriptions-item>
                <el-descriptions-item label="上周期运行时长">{{ cycleData.lastCycleLen }}秒</el-descriptions-item>
                <el-descriptions-item label="当前周期过渡调整标记">
                  {{ cycleData.adjustFlag === 1 ? '过渡调整中' : '无过渡调整或已完成' }}
                </el-descriptions-item>
                <el-descriptions-item label="操作">
                  <el-button 
                    type="primary" 
                    size="small" 
                    @click="showOriginalJson('cycleData')"
                  >
                    查看原始JSON
                  </el-button>
                </el-descriptions-item>
               
              </el-descriptions>
                  </div>
                </div>
              </div>
            </el-card>

            <!-- 信号配时方案 -->
            <el-card class="info-card" v-if="signalData">
              <template #header>
                <div class="card-header">
                  <span>信号配时方案（2001）</span>
                  <el-button 
                    type="primary" 
                    size="small" 
                    @click="showOriginalJson('signalData')"
                  >
                    查看原始JSON
                  </el-button>
                </div>
              </template>
              <el-descriptions :column="2" border>
                <el-descriptions-item label="当前方案开始时间">{{ signalData.curPlanStartTime }}</el-descriptions-item>
                <el-descriptions-item label="当前方案号">{{ signalData.curPlan?.planNo }}</el-descriptions-item>
                <el-descriptions-item label="下一方案开始时间">{{ signalData.nextPlanStartTime }}</el-descriptions-item>
                <el-descriptions-item label="下一方案号">{{ signalData.nextPlan?.planNo }}</el-descriptions-item>
              </el-descriptions>

             

              <!-- 当前方案灯组信息 -->
              <div class="plan-section">
                <h4>当前方案灯组信息(2001)</h4>
                <div class="filter-section">
                  <el-checkbox-group v-model="selectedLampTypes" @change="handleLampTypeFilter">
                    <el-checkbox v-for="(text, value) in lampGroupDict" 
                               :key="value" 
                               :label="value">
                      {{ text }}
                    </el-checkbox>
                  </el-checkbox-group>
                </div>
                <el-table :data="filteredLampGroups" border style="width: 100%">
                  <el-table-column prop="controlDir" label="控制方向" width="100"/>
                  <el-table-column prop="lampType" label="灯组类型" width="200">
                    <template #default="scope">
                      {{ getLampGroupText(scope.row.lampType) }} ({{ scope.row.lampType }})
                    </template>
                  </el-table-column>
                  <el-table-column label="色步信息" min-width="300">
                    <template #default="scope">
                      <div class="steps-container">
                        <div v-for="(step, stepIndex) in scope.row.colorStepList" :key="stepIndex" 
                             class="step-bar" 
                             :style="getStepBarStyle(step, scope.row)"
                             :title="`${getLampStatusText(step.lampStatus)} - ${step.duration}秒`">
                          <div class="step-duration">{{ step.duration }}s</div>
                        </div>
                      </div>
                    </template>
                  </el-table-column>
                </el-table>
              </div>

              <!-- 下一方案灯组信息 -->
              <div class="plan-section" v-if="signalData.nextPlan">
                <h4>下一方案灯组信息(2001)</h4>
               
                <el-table :data="filteredNextLampGroups" border style="width: 100%">
                  <el-table-column prop="controlDir" label="控制方向" width="100"/>
                  <el-table-column prop="lampType" label="灯组类型" width="200">
                    <template #default="scope">
                      {{ getLampGroupText(scope.row.lampType) }} ({{ scope.row.lampType }})
                    </template>
                  </el-table-column>
                  <el-table-column label="色步信息" min-width="300">
                    <template #default="scope">
                      <div class="steps-container">
                        <div v-for="(step, stepIndex) in scope.row.colorStepList" :key="stepIndex" 
                             class="step-bar" 
                             :style="getStepBarStyle(step, scope.row)"
                             :title="`${getLampStatusText(step.lampStatus)} - ${step.duration}秒`">
                          <div class="step-duration">{{ step.duration }}s</div>
                        </div>
                      </div>
                    </template>
                  </el-table-column>
                </el-table>
              </div>
            </el-card>
          </div>
        </div>
      </el-main>
    </el-container>
  </div>
</template>

<script>
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useRouter } from 'vue-router'
import CrossMap from '../components/CrossMap.vue'
import { ElMessageBox } from 'element-plus'

export default {
  name: 'GBParamCheck',
  components: {
    CrossMap
  },
  setup() {
    const router = useRouter()
    const keys = ref([])
    const crossIds = ref([])
    const searchKeyword = ref('')
    const loading = ref(true)
    const activeCrossId = ref('')
    const crossData = ref(null)
    const signalData = ref(null)
    const cycleData = ref(null)
    const selectedLampTypes = ref([])
    const lampGroupDict = {
      '10': '机动车信号灯',
      '21': '机动车方向指示信号灯-直行',
      '22': '机动车方向指示信号灯-左转',
      '23': '机动车方向指示信号灯-右转',
      '30': '非机动车信号灯',
      '32': '非机动车信号灯-左转',
      '40': '人行横道信号灯',
      '41': '人行横道信号灯-进口',
      '42': '人行横道信号灯-出口',
      '50': '车道信号灯',
      '61': '有轨电车信号灯-直行',
      '62': '有轨电车信号灯-左转',
      '63': '有轨电车信号灯-右转',
      '70': '公交信号灯',
      '80': '掉头信号灯',
      '99': '其他'
    }
    const validationErrors = ref([])

    const filteredLampGroups = computed(() => {
      if (!signalData.value?.curPlan?.lampColorStepPlanList) return []
      if (selectedLampTypes.value.length === 0) return signalData.value.curPlan.lampColorStepPlanList
      return signalData.value.curPlan.lampColorStepPlanList.filter(lamp => 
        selectedLampTypes.value.includes(lamp.lampType.toString())
      )
    })

    const filteredNextLampGroups = computed(() => {
      if (!signalData.value?.nextPlan?.lampColorStepPlanList) return []
      if (selectedLampTypes.value.length === 0) return signalData.value.nextPlan.lampColorStepPlanList
      return signalData.value.nextPlan.lampColorStepPlanList.filter(lamp => 
        selectedLampTypes.value.includes(lamp.lampType.toString())
      )
    })

    const filteredCrossIds = computed(() => {
      return crossIds.value
    })

    const handleLampTypeFilter = () => {
      // 可以在这里添加额外的过滤逻辑
    }

    // 搜索处理函数
    const handleSearch = async () => {
      console.log('搜索关键字:', searchKeyword.value)
      try {
        if (!searchKeyword.value) {
          // 如果搜索框为空，获取所有键
          await fetchKeys()
          return
        }
        
        // 使用 search_keys 函数搜索
        const searchPattern = `V2X:1001:${searchKeyword.value}*`
        const response = await invoke('search_keys', { pattern: searchPattern })
        console.log('搜索响应:', response)
        
        if (response.success) {
          const matchedKeys = response.data
          console.log('匹配到的键:', matchedKeys)
          
          // 更新列表
          keys.value = matchedKeys
          crossIds.value = matchedKeys.map(key => ({
            crossId: key.split(':')[2]
          }))
          
          // 如果只匹配到一个结果，自动选中
          if (matchedKeys.length === 1) {
            handleSelect(crossIds.value[0].crossId)
          }
        } else {
          console.error('搜索失败:', response.error)
        }
      } catch (error) {
        console.error('搜索失败:', error)
      }
    }

    const fetchKeys = async () => {
      try {
        // 获取键列表
        const response = await invoke('search_keys', { pattern: 'V2X:1001:*' })
        if (response.success) {
          keys.value = response.data
          crossIds.value = keys.value.map(key => ({
            crossId: key.split(':')[2]
          }))
        } else {
          console.error('获取键失败:', response.error)
        }
      } catch (error) {
        console.error('获取键失败:', error)
      } finally {
        loading.value = false
      }
    }

    const removeError = (index) => {
      validationErrors.value.splice(index, 1)
    }

    const showError = async (message) => {
      validationErrors.value.push(message)
      console.error(message)
    }

    const showInfo = async (message) => {
      console.info(message)
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
          const errorMsg = `[1001]${label}(${field})为必填项，当前值：${data[field]}`
          errors.push(errorMsg)
          await showError(errorMsg)
        }
      }
      // 验证recordTime格式
      if (data.recordTime && !/^[0-9]{14}$/.test(data.recordTime)) {
        const errorMsg = `[1001]数据更新时间(recordTime)格式错误，应为YYYYMMDDhhmmss格式，当前值：${data.recordTime}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }
      // 验证position格式
      if (data.position && !/^-?\d+(\.\d+)?,-?\d+(\.\d+)?$/.test(data.position)) {
        const errorMsg = `[1001]交叉口中心位置坐标(position)格式错误，应为经度,纬度格式，当前值：${data.position}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }
      // 验证车道数据
      if (data.lanes && Array.isArray(data.lanes)) {
        // 检查是否缺少必要类型的车道
        const laneTypes = data.lanes.map(lane => lane.type)
        const missingTypes = []
        if (!laneTypes.includes(1)) {
          missingTypes.push('机动车车道')
        }
        
        // 检查是否有出口道
        const hasExitLane = data.lanes.some(lane => lane.attr === 1 || lane.attr === '1')
        if (!hasExitLane) {
          missingTypes.push('机动车出口道')
        }
        if (missingTypes.length > 0) {
          const errorMsg = `[1001]缺少以下类型的车道：${missingTypes.join('、')}`
          errors.push(errorMsg)
          await showError(errorMsg)
        }
        // 按方向+属性分组检查车道航向角
        const directionAttrGroups = new Map()
        // 检查每个车道的详细信息
        for (let index = 0; index < data.lanes.length; index++) {
          const lane = data.lanes[index];
          // 检查车道属性
          if (lane.attr === undefined || lane.attr === null || lane.attr === '') {
            const errorMsg = `[1001]第${index + 1}条车道的属性(attr)为必填项`
            errors.push(errorMsg)
            await showError(errorMsg)
          } else if (!['0','1','2','3','9',0,1,2,3,9].includes(lane.attr)) {
            const errorMsg = `[1001]第${index + 1}条车道的属性(attr)值无效，当前值：${lane.attr}`
            errors.push(errorMsg)
            await showError(errorMsg)
          }
          // 检查车道航向角
          if (lane.angle === undefined || lane.angle === null || lane.angle === '') {
            const errorMsg = `[1001]第${index + 1}条车道的行车航向角(angle)为必填项`
            errors.push(errorMsg)
            await showError(errorMsg)
          } else {
            const angle = Number(lane.angle)
            if (isNaN(angle)) {
              const errorMsg = `[1001]第${index + 1}条车道的行车航向角(angle)必须为数字，当前值：${lane.angle}`
              errors.push(errorMsg)
              await showError(errorMsg)
            } else if (angle < 0 || angle > 360) {
              const errorMsg = `[1001]第${index + 1}条车道的行车航向角(angle)必须在0-360度之间，当前值：${angle}`
              errors.push(errorMsg)
              await showError(errorMsg)
            }
            // 将车道按方向+属性分组（只对非匝道和非路段车道的车道进行分组）
            if (lane.direction !== undefined && lane.direction !== null && lane.direction !== '' && lane.attr !== 2 && lane.attr !== 3 && lane.attr !== '2' && lane.attr !== '3') {
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
        }
        // 检查每个方向+属性下的车道航向角是否相同
        for (const [groupKey, lanes] of directionAttrGroups.entries()) {
          const angles = new Set(lanes.map(lane => lane.angle))
          if (angles.size > 1) {
            const laneIndices = lanes.map(lane => lane.index).join('、')
            const [direction, attr] = groupKey.split('_')
            const errorMsg = `[1001]方向${direction}属性${attr}下的车道（第${laneIndices}条）航向角不一致，存在${angles.size}个不同的值：${Array.from(angles).join('、')}`
            errors.push(errorMsg)
            await showError(errorMsg)
          }
        }
      }
      // 验证人行横道数据
      if (data.peds && Array.isArray(data.peds)) {
        if (data.peds.length === 0) {
          const errorMsg = `[1001]缺少人行横道数据`
          errors.push(errorMsg)
          await showError(errorMsg)
        } else {
          // for (let index = 0; index < data.peds.length; index++) {
          //   const ped = data.peds[index];
          //   // 检查人行横道位置坐标（中心线起点和终点，GCJ-02坐标系，经纬度用逗号分隔，多个点用分号分隔）
          //   if (!ped.positions) {
          //     const errorMsg = `[1001]第${index + 1}条人行横道缺少位置坐标(positions)`
          //     errors.push(errorMsg)
          //     await showError(errorMsg)
          //   } else {
          //     // 检查格式：经度,纬度;经度,纬度
          //     const posArr = ped.positions.split(';')
          //     if (posArr.length < 2) {
          //       const errorMsg = `[1001]第${index + 1}条人行横道位置坐标(positions)格式错误，应至少包含起点和终点（经度,纬度;经度,纬度）`
          //       errors.push(errorMsg)
          //       await showError(errorMsg)
          //     } else {
          //       for (let i = 0; i < posArr.length; i++) {
          //         if (!/^-?\d+(\.\d+)?,-?\d+(\.\d+)?$/.test(posArr[i].trim())) {
          //           const errorMsg = `[1001]第${index + 1}条人行横道位置坐标(positions)第${i + 1}个点格式错误，应为经度,纬度格式`
          //           errors.push(errorMsg)
          //           await showError(errorMsg)
          //         }
          //       }
          //     }
          //   }
          // }
        }
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
          const errorMsg = `[2002]信号周期开始数据中${label}(${field})为必填项，当前值：${data[field]}`
          errors.push(errorMsg)
          await showError(errorMsg)
        }
      }

      // 验证recordTime格式
      if (data.recordTime && !/^\d{14}$/.test(data.recordTime)) {
        const errorMsg = `[2002]信号周期开始数据更新时间(recordTime)格式错误，应为YYYYMMDDhhmmss格式，当前值：${data.recordTime}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }

      // 验证curCycleStartTime格式
      if (data.curCycleStartTime && !/^\d{14}$/.test(data.curCycleStartTime)) {
        const errorMsg = `[2002]当前周期开始时间(curCycleStartTime)格式错误，应为YYYYMMDDhhmmss格式，当前值：${data.curCycleStartTime}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }

      // 验证lastCycleLen范围
      if (data.lastCycleLen !== undefined && (data.lastCycleLen < 0 || data.lastCycleLen > 500)) {
        const errorMsg = `[2002]上周期运行时长(lastCycleLen)必须在0-500秒之间，当前值：${data.lastCycleLen}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }

      // 验证adjustFlag值
      // if (data.adjustFlag !== undefined && ![0, 1].includes(data.adjustFlag)) {
      //   const errorMsg = `[2002]当前周期过渡调整标记(adjustFlag)必须为0或1，当前值：${data.adjustFlag}`
      //   errors.push(errorMsg)
      //   await showError(errorMsg)
      // }

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
          const errorMsg = `[2003]信号灯组灯色状态数据中${label}(${field})为必填项，当前值：${data[field]}`
          errors.push(errorMsg)
          await showError(errorMsg)
        }
      }

      // 验证recordTime格式
      if (data.recordTime && !/^\d{14}(\d{3})?$/.test(data.recordTime)) {
        const errorMsg = `[2003]信号灯组灯色状态数据更新时间(recordTime)格式错误，应为YYYYMMDDhhmmss或YYYYMMDDhhmmssSSS格式，当前值：${data.recordTime}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }

      // 验证timestamp是否为有效的时间戳
      if (data.timestamp !== undefined && (isNaN(data.timestamp) || data.timestamp < 0)) {
        const errorMsg = `[2003]时间戳(timestamp)必须为非负整数，当前值：${data.timestamp}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }

      // 验证lampStatusList
      if (data.lampStatusList && Array.isArray(data.lampStatusList)) {
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
              const errorMsg = `[2003]第${index + 1}个灯组的${label}(${field})为必填项，当前值：${lamp[field]}`
              errors.push(errorMsg)
              await showError(errorMsg)
            }
          }

          // 验证controlDir范围（航向角）
          if (lamp.controlDir !== undefined && (lamp.controlDir < 0 || lamp.controlDir > 360)) {
            const errorMsg = `[2003]第${index + 1}个灯组的进口方向(controlDir)必须在0-360度之间，当前值：${lamp.controlDir}`
            errors.push(errorMsg)
            await showError(errorMsg)
          }

          // 验证lampType值
          const validLampTypes = ['10', '21', '22', '23', '30', '32', '40', '41', '42', '50', '61', '62', '63', '70', '80', '99']
          if (lamp.lampType !== undefined && !validLampTypes.includes(lamp.lampType.toString())) {
            const errorMsg = `[2003]第${index + 1}个灯组的灯组类型(lampType)值无效，当前值：${lamp.lampType}，有效值：${validLampTypes.join(', ')}`
            errors.push(errorMsg)
            await showError(errorMsg)
          }

          // 验证lampStatus值
          const validLampStatus = ['11', '21', '22', '23', '42', '43']
          if (lamp.lampStatus !== undefined && !validLampStatus.includes(lamp.lampStatus.toString())) {
            const errorMsg = `[2003]第${index + 1}个灯组的灯色状态(lampStatus)值无效，当前值：${lamp.lampStatus}，有效值：${validLampStatus.join(', ')}`
            errors.push(errorMsg)
            await showError(errorMsg)
          }

          // 验证remainTime范围
          if (lamp.remainTime !== undefined && (lamp.remainTime < 0 || lamp.remainTime > 999)) {
            const errorMsg = `[2003]第${index + 1}个灯组的剩余时长(remainTime)必须在0-999秒之间，当前值：${lamp.remainTime}`
            errors.push(errorMsg)
            await showError(errorMsg)
          }
        })
      } else {
        const errorMsg = `[2003]灯组灯色状态列表(lampStatusList)必须为数组，当前值：${typeof data.lampStatusList}`
        errors.push(errorMsg)
        await showError(errorMsg)
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
        if (data[field] === undefined || data[field] === null || data[field] === '') {
          const errorMsg = `[2001]信号配时方案数据中${label}(${field})为必填项，当前值：${data[field]}`
          errors.push(errorMsg)
          await showError(errorMsg)
        }
      }

      // 验证recordTime格式
      if (data.recordTime && !/^\d{14}$/.test(data.recordTime)) {
        const errorMsg = `[2001]数据更新时间(recordTime)格式错误，应为YYYYMMDDhhmmss格式，当前值：${data.recordTime}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }

      // 验证crossName长度
      if (data.crossName && data.crossName.length > 50) {
        const errorMsg = `[2001]交叉口名称(crossName)最大长度为50，当前长度：${data.crossName.length}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }

      // 验证position格式
      if (data.position && !/^-?\d+(\.\d+)?,-?\d+(\.\d+)?$/.test(data.position)) {
        const errorMsg = `[2001]交叉口中心位置坐标(position)格式错误，应为经度,纬度格式，当前值：${data.position}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }

      // 验证curPlanStartTime格式
      if (data.curPlanStartTime && !/^\d{6}$/.test(data.curPlanStartTime)) {
        const errorMsg = `[2001]当前方案时段开始时间(curPlanStartTime)格式错误，应为hhmmss格式，当前值：${data.curPlanStartTime}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }

      // 验证nextPlanStartTime格式（如果存在）
      if (data.nextPlanStartTime && !/^\d{6}$/.test(data.nextPlanStartTime)) {
        const errorMsg = `[2001]计划下个方案时段开始时间(nextPlanStartTime)格式错误，应为hhmmss格式，当前值：${data.nextPlanStartTime}`
        errors.push(errorMsg)
        await showError(errorMsg)
      }

      // 验证当前方案
      if (data.curPlan) {
        // 验证方案类型
        if (data.curPlan.type !== 1 && data.curPlan.type !== 2) {
          const errorMsg = `[2001]当前方案类型(type)必须为1(固定配时)或2(非固定配时)，当前值：${data.curPlan.type}`
          errors.push(errorMsg)
          await showError(errorMsg)
        }

        // 验证方案号
        if (data.curPlan.planNo !== undefined && (data.curPlan.planNo < 0 || data.curPlan.planNo > 200)) {
          const errorMsg = `[2001]当前方案号(planNo)必须在0-200之间，当前值：${data.curPlan.planNo}`
          errors.push(errorMsg)
          await showError(errorMsg)
        }

        // 验证灯组色步方案信息列表
        if (!data.curPlan.lampColorStepPlanList || !Array.isArray(data.curPlan.lampColorStepPlanList)) {
          const errorMsg = `[2001]当前方案灯组色步方案信息列表(lampColorStepPlanList)必须为数组，当前值：${typeof data.curPlan.lampColorStepPlanList}`
          errors.push(errorMsg)
          await showError(errorMsg)
        } else {
          // 检查所有灯组的duration之和是否相等
          const durationSums = data.curPlan.lampColorStepPlanList.map(lamp => {
            return lamp.colorStepList.reduce((sum, step) => sum + step.duration, 0)
          })
          
          const firstSum = durationSums[0]
          const hasUnequalSums = durationSums.some((sum, index) => {
            if (sum !== firstSum) {
              const errorMsg = `[2001]当前方案中第${index + 1}个灯组的色步时长之和(${sum}秒)与第1个灯组(${firstSum}秒)不相等`
              errors.push(errorMsg)
              showError(errorMsg)
              return true
            }
            return false
          })

          data.curPlan.lampColorStepPlanList.forEach(async (lamp, index) => {
            // 验证灯组指示的进口方向
            if (lamp.controlDir === undefined || lamp.controlDir < 0 || lamp.controlDir > 360) {
              const errorMsg = `[2001]第${index + 1}个灯组的进口方向(controlDir)必须在0-360度之间，当前值：${lamp.controlDir}`
              errors.push(errorMsg)
              await showError(errorMsg)
            }

            // 验证灯组类型
            const validLampTypes = ['10', '21', '22', '23', '30', '32', '40', '41', '42', '50', '61', '62', '63', '70', '80', '99']
            if (lamp.lampType === undefined || !validLampTypes.includes(lamp.lampType.toString())) {
              const errorMsg = `[2001]第${index + 1}个灯组的灯组类型(lampType)值无效，当前值：${lamp.lampType}，有效值：${validLampTypes.join(', ')}`
              errors.push(errorMsg)
              await showError(errorMsg)
            }

            // 验证色步信息列表
            if (!lamp.colorStepList || !Array.isArray(lamp.colorStepList)) {
              const errorMsg = `[2001]第${index + 1}个灯组的色步信息列表(colorStepList)必须为数组，当前值：${typeof lamp.colorStepList}`
              errors.push(errorMsg)
              await showError(errorMsg)
            } else {
              lamp.colorStepList.forEach(async (step, stepIndex) => {
                // 验证色步号
                if (step.stepNo === undefined || step.stepNo < 1 || step.stepNo > 999) {
                  const errorMsg = `[2001]第${index + 1}个灯组的第${stepIndex + 1}个色步的色步号(stepNo)必须在1-999之间，当前值：${step.stepNo}`
                  errors.push(errorMsg)
                  await showError(errorMsg)
                }

                // 验证灯色状态
                const validLampStatus = ['11', '21', '22', '23', '42', '43']
                if (step.lampStatus === undefined || !validLampStatus.includes(step.lampStatus.toString())) {
                  const errorMsg = `[2001]第${index + 1}个灯组的第${stepIndex + 1}个色步的灯色状态(lampStatus)值无效，当前值：${step.lampStatus}，有效值：${validLampStatus.join(', ')}`
                  errors.push(errorMsg)
                  await showError(errorMsg)
                }

                // 验证持续时长
                if (step.duration === undefined || !Number.isInteger(step.duration) || step.duration < 0) {
                  const errorMsg = `[2001]第${index + 1}个灯组的第${stepIndex + 1}个色步的持续时长(duration)必须为非负整数，当前值：${step.duration}`
                  errors.push(errorMsg)
                  await showError(errorMsg)
                }
              })
            }
          })
        }
      }

      // 验证下个方案（如果存在）
      if (data.nextPlan) {
        // 验证方案类型
        if (data.nextPlan.type !== 1 && data.nextPlan.type !== 2) {
          const errorMsg = `[2001]下个方案类型(type)必须为1(固定配时)或2(非固定配时)，当前值：${data.nextPlan.type}`
          errors.push(errorMsg)
          await showError(errorMsg)
        }

        // 验证方案号
        if (data.nextPlan.planNo !== undefined && (data.nextPlan.planNo < 0 || data.nextPlan.planNo > 200)) {
          const errorMsg = `[2001]下个方案号(planNo)必须在0-200之间，当前值：${data.nextPlan.planNo}`
          errors.push(errorMsg)
          await showError(errorMsg)
        }

        // 验证灯组色步方案信息列表
        if (!data.nextPlan.lampColorStepPlanList || !Array.isArray(data.nextPlan.lampColorStepPlanList)) {
          const errorMsg = `[2001]下个方案灯组色步方案信息列表(lampColorStepPlanList)必须为数组，当前值：${typeof data.nextPlan.lampColorStepPlanList}`
          errors.push(errorMsg)
          await showError(errorMsg)
        } else {
          // 检查所有灯组的duration之和是否相等
          const durationSums = data.nextPlan.lampColorStepPlanList.map(lamp => {
            return lamp.colorStepList.reduce((sum, step) => sum + step.duration, 0)
          })
          
          const firstSum = durationSums[0]
          const hasUnequalSums = durationSums.some((sum, index) => {
            if (sum !== firstSum) {
              const errorMsg = `[2001]下个方案中第${index + 1}个灯组的色步时长之和(${sum}秒)与第1个灯组(${firstSum}秒)不相等`
              errors.push(errorMsg)
              showError(errorMsg)
              return true
            }
            return false
          })

          data.nextPlan.lampColorStepPlanList.forEach(async (lamp, index) => {
            // 验证灯组指示的进口方向
            if (lamp.controlDir === undefined || lamp.controlDir < 0 || lamp.controlDir > 360) {
              const errorMsg = `[2001]下个方案第${index + 1}个灯组的进口方向(controlDir)必须在0-360度之间，当前值：${lamp.controlDir}`
              errors.push(errorMsg)
              await showError(errorMsg)
            }

            // 验证灯组类型
            const validLampTypes = ['10', '21', '22', '23', '30', '32', '40', '41', '42', '50', '61', '62', '63', '70', '80', '99']
            if (lamp.lampType === undefined || !validLampTypes.includes(lamp.lampType.toString())) {
              const errorMsg = `[2001]下个方案第${index + 1}个灯组的灯组类型(lampType)值无效，当前值：${lamp.lampType}，有效值：${validLampTypes.join(', ')}`
              errors.push(errorMsg)
              await showError(errorMsg)
            }

            // 验证色步信息列表
            if (!lamp.colorStepList || !Array.isArray(lamp.colorStepList)) {
              const errorMsg = `[2001]下个方案第${index + 1}个灯组的色步信息列表(colorStepList)必须为数组，当前值：${typeof lamp.colorStepList}`
              errors.push(errorMsg)
              await showError(errorMsg)
            } else {
              lamp.colorStepList.forEach(async (step, stepIndex) => {
                // 验证色步号
                if (step.stepNo === undefined || step.stepNo < 1 || step.stepNo > 999) {
                  const errorMsg = `[2001]下个方案第${index + 1}个灯组的第${stepIndex + 1}个色步的色步号(stepNo)必须在1-999之间，当前值：${step.stepNo}`
                  errors.push(errorMsg)
                  await showError(errorMsg)
                }

                // 验证灯色状态
                const validLampStatus = ['11', '21', '22', '23', '42', '43']
                if (step.lampStatus === undefined || !validLampStatus.includes(step.lampStatus.toString())) {
                  const errorMsg = `[2001]下个方案第${index + 1}个灯组的第${stepIndex + 1}个色步的灯色状态(lampStatus)值无效，当前值：${step.lampStatus}，有效值：${validLampStatus.join(', ')}`
                  errors.push(errorMsg)
                  await showError(errorMsg)
                }

                // 验证持续时长
                if (step.duration === undefined || !Number.isInteger(step.duration) || step.duration < 0) {
                  const errorMsg = `[2001]下个方案第${index + 1}个灯组的第${stepIndex + 1}个色步的持续时长(duration)必须为非负整数，当前值：${step.duration}`
                  errors.push(errorMsg)
                  await showError(errorMsg)
                }
              })
            }
          })
        }
      }

      return errors
    }

    const handleSelect = async (index) => {
      activeCrossId.value = index
      // 清空错误信息
      validationErrors.value = []
      try {
        // 获取路口基本信息
        const crossDataKey = `V2X:1001:${index}`
        console.log("crossDataKey:",crossDataKey)
        const crossDataResponse = await invoke('get_key_data', { key: crossDataKey })
        console.log(crossDataResponse)
        if (crossDataResponse.success) {
          crossData.value = JSON.parse(crossDataResponse.data.value)
          // 验证数据
          const errors = await validateCrossData(crossData.value)
          if (errors.length > 0) {
            await showError(`路口${index}数据验证发现${errors.length}个问题`)
          } else {
            await showInfo(`路口${index}数据验证通过`)
          }
        } else {
          const errorMsg = `获取路口基本信息失败: ${crossDataResponse.error}`
          await showError(errorMsg)
        }

        // 获取信号配时方案
        const signalDataKey = `V2X:2001:${index}`
        const signalDataResponse = await invoke('get_key_data', { key: signalDataKey })
        if (signalDataResponse.success) {
          signalData.value = JSON.parse(signalDataResponse.data.value)
          // 验证数据
          const errors = await validateSignalData(signalData.value)
          if (errors.length > 0) {
            await showError(`路口${index}信号配时方案数据验证发现${errors.length}个问题`)
          } else {
            await showInfo(`路口${index}信号配时方案数据验证通过`)
          }
        } else {
          const errorMsg = `获取信号配时方案失败: ${signalDataResponse.error}`
          await showError(errorMsg)
        }

        // 获取信号灯组灯色状态
        const lampStatusKey = `V2X:2003:${index}`
        const lampStatusResponse = await invoke('get_key_data', { key: lampStatusKey })
        if (lampStatusResponse.success) {
          const lampStatusData = JSON.parse(lampStatusResponse.data.value)
          crossData.value.lampStatusList = lampStatusData.lampStatusList.sort((a, b) => a.controlDir - b.controlDir)
          // 验证数据
          const errors = await validateLampStatusData(lampStatusData)
          if (errors.length > 0) {
            await showError(`路口${index}信号灯组灯色状态数据验证发现${errors.length}个问题`)
          } else {
            await showInfo(`路口${index}信号灯组灯色状态数据验证通过`)
          }
        } else {
          const errorMsg = `获取灯组灯色状态失败: ${lampStatusResponse.error}`
          await showError(errorMsg)
        }

        // 获取信号周期开始数据
        const cycleDataKey = `V2X:2002:${index}`
        const cycleDataResponse = await invoke('get_key_data', { key: cycleDataKey })
        if (cycleDataResponse.success) {
          cycleData.value = JSON.parse(cycleDataResponse.data.value)
          // 验证数据
          const errors = await validateCycleData(cycleData.value)
          if (errors.length > 0) {
            await showError(`路口${index}信号周期开始数据验证发现${errors.length}个问题`)
          } else {
            await showInfo(`路口${index}信号周期开始数据验证通过`)
          }
        } else {
          const errorMsg = `获取信号周期开始数据失败: ${cycleDataResponse.error}`
          await showError(errorMsg)
        }
      } catch (error) {
        await showError(`获取数据失败: ${error.message}`)
      }
    }

    const getDirectionText = (direction) => {
      const directionDict = {
        '1': '北',
        '2': '东北',
        '3': '东',
        '4': '东南',
        '5': '南',
        '6': '西南',
        '7': '西',
        '8': '西北',
        '9': '其他'
      }
      return directionDict[direction] || `未知方向(${direction})`
    }

    const getLampStatusText = (status) => {
      const statusDict = {
        '11': '灭灯',
        '21': '红灯',
        '22': '黄灯',
        '23': '绿灯',
        '42': '黄闪',
        '43': '绿闪'
      }
      return statusDict[status] || `未知(${status})`
    }

    const getLampGroupText = (type) => {
      const lampGroupDict = {
        '10': '机动车信号灯',
        '21': '机动车方向指示信号灯-直行',
        '22': '机动车方向指示信号灯-左转',
        '23': '机动车方向指示信号灯-右转',
        '30': '非机动车信号灯',
        '32': '非机动车信号灯-左转',
        '40': '人行横道信号灯',
        '41': '人行横道信号灯-进口',
        '42': '人行横道信号灯-出口',
        '50': '车道信号灯',
        '61': '有轨电车信号灯-直行',
        '62': '有轨电车信号灯-左转',
        '63': '有轨电车信号灯-右转',
        '70': '公交信号灯',
        '80': '掉头信号灯',
        '99': '其他'
      }
      return lampGroupDict[type] || `未知灯组(${type})`
    }

    const getAttributeText = (attr) => {
      switch (attr) {
        case 0: return '路口进口'
        case 1: return '路口出口'
        case 2: return '匝道'
        case 3: return '路段车道'
        case 9: return '其他'
        default: return '未知'
      }
    }

    const getFeatureText = (type) => {
      switch (type) {
        case 1: return '机动车车道'
        case 2: return '非机动车车道'
        case 3: return '机非混合车道'
        case 4: return '行人便道'
        case 9: return '其他'
        default: return '未知'
      }
    }

    const getMovementText = (turn) => {
      switch (turn) {
        case 11: return '直行'
        case 12: return '左转'
        case 13: return '右转'
        case 21: return '直左混行'
        case 22: return '直右混行'
        case 23: return '左右混行'
        case 24: return '直左右混行'
        case 31: return '掉头'
        case 32: return '掉头加左转'
        case 33: return '掉头加直行'
        case 34: return '掉头加右转'
        case 99: return '其他'
        default: return '未知'
      }
    }

    const getPedAttrText = (attr) => {
      switch (attr) {
        case 1:
          return '一次过街'
        case 21:
          return '二次过街-交叉口进口'
        case 22:
          return '二次过街-交叉口出口'
        default:
          return '未知'
      }
    }

    const getStepBarStyle = (step, lampGroup) => {
      const colorMap = {
        '11': '#666666', // 灭灯
        '21': '#ff4d4f', // 红灯
        '22': '#faad14', // 黄灯
        '23': '#52c41a', // 绿灯
        '42': '#faad14', // 黄闪
        '43': '#52c41a'  // 绿闪
      }
      
      // 计算宽度：每个色步的宽度根据其持续时间占总时间的比例
      const totalDuration = lampGroup.colorStepList.reduce((sum, s) => sum + s.duration, 0)
      const width = (step.duration / totalDuration * 100) + '%'
      const backgroundColor = colorMap[step.lampStatus] || '#666666'
      
      return {
        width,
        backgroundColor,
        border: '2px solid rgba(255, 255, 255, 0.3)'
      }
    }

    const goToLogin = async () => {
      try {
        // 跳转到登录页面
        router.push({
          name: 'Login'
        })
      } catch (e) {
        console.error('跳转登录页面失败:', e)
        ElMessage.error('跳转失败')
      }
    }

    const goToBatchCheck = () => {
      router.push({
        name: 'GBParamBatchCheck'
      })
    }

    const showOriginalJson = (type) => {
      let jsonData = null
      let title = ''
      
      switch(type) {
        case 'crossData':
          // 创建一个新对象，排除lampStatusList字段
          const { lampStatusList, ...crossDataWithoutLampStatus } = crossData.value
          jsonData = crossDataWithoutLampStatus
          title = '路口基本信息原始JSON'
          break
        case 'signalData':
          jsonData = signalData.value
          title = '信号配时方案原始JSON'
          break
        case 'cycleData':
          jsonData = cycleData.value
          title = '信号周期信息原始JSON'
          break
          case 'lampStatusData':
            jsonData= crossData.value.lampStatusList
            title = '灯组灯色状态原始JSON'
            break
      }

      if (jsonData) {
        ElMessageBox({
          message: `<div class="json-content"><pre>${JSON.stringify(jsonData, null, 2)}</pre></div>`,
          title,
          dangerouslyUseHTMLString: true,
          customClass: 'json-dialog',
          confirmButtonText: '关闭',
          closeOnClickModal: true,
          showCancelButton: false,
          showClose: true
        })
      }
    }

    onMounted(() => {
      fetchKeys()
    })

    return {
      keys,
      crossIds,
      searchKeyword,
      filteredCrossIds,
      handleSearch,
      loading,
      activeCrossId,
      crossData,
      signalData,
      cycleData,
      handleSelect,
      getDirectionText,
      getAttributeText,
      getFeatureText,
      getMovementText,
      getPedAttrText,
      getLampStatusText,
      getLampGroupText,
      getStepBarStyle,
      selectedLampTypes,
      lampGroupDict,
      filteredLampGroups,
      filteredNextLampGroups,
      handleLampTypeFilter,
      validationErrors,
      removeError,
      goToLogin,
      goToBatchCheck,
      showOriginalJson,
      validateCycleData,
      validateLampStatusData
    }
  }
}
</script>

<style scoped>
.gb-param-check {
  height: 100%;
}

.aside {
  background-color: #f5f7fa;
  border-right: 1px solid #e6e6e6;
  padding: 20px 0;
  display: flex;
  flex-direction: column;
}

.aside-header {
  padding: 0 20px;
  margin-bottom: 15px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.aside-header h3 {
  margin: 0;
  color: #303133;
  font-size: 15px;
}

.header-buttons {
  display: flex;
  gap: 10px;
}

.batch-check-btn {
  font-size: 12px;
  padding: 6px 12px;
}

.login-btn {
  font-size: 12px;
  padding: 6px 12px;
}

.search-box {
  padding: 0 20px;
  margin-bottom: 15px;
}

.search-box .el-input {
  width: 100%;
}

.cross-list {
  flex: 1;
  padding: 0 10px;
}

.cross-item {
  padding: 8px 15px;
  margin: 2px 0;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.3s;
  color: #606266;
  background-color: #fff;
  border: 1px solid #e6e6e6;
  user-select: none;
  font-size: 13px;
  line-height: 1.4;
}

.cross-item:hover {
  background-color: #f0f2f5;
  color: #409eff;
  border-color: #409eff;
}

.cross-item.active {
  background-color: #ecf5ff;
  color: #409eff;
  border-color: #409eff;
  font-weight: 500;
}

.el-main {
  padding: 20px;
}

.loading {
  text-align: center;
  margin-top: 20px;
  color: #666;
}

.no-data {
  text-align: center;
  margin-top: 20px;
  color: #999;
}

.content {
  margin-top: 20px;
}

.info-card {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.cross-detail {
  padding: 20px;
}

.plan-section {
  margin-top: 20px;
}

.plan-section h4 {
  margin-bottom: 10px;
  color: #606266;
}

.step-visualization-container {
  padding: 20px;
  background-color: #f5f7fa;
  border-radius: 4px;
}

.lamp-group {
  margin-bottom: 20px;
  padding: 10px;
  background-color: white;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.lamp-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 10px;
  padding: 5px 0;
  border-bottom: 1px solid #ebeef5;
}

.direction {
  font-weight: bold;
  color: #303133;
}

.lamp-type {
  color: #606266;
}

.steps-container {
  display: flex;
  gap: 2px;
  padding: 10px 0;
  overflow-x: auto;
  background-color: rgba(0, 0, 0, 0.1);
  border-radius: 4px;
}

.step-bar {
  height: 30px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 12px;
  min-width: 40px;
  transition: all 0.3s;
  cursor: pointer;
}

.step-duration {
  font-weight: bold;
  text-shadow: 0 0 2px rgba(0, 0, 0, 0.5);
}

.filter-section {
  margin-bottom: 20px;
  padding: 15px;
  background-color: #f5f7fa;
  border-radius: 4px;
}

.filter-section .el-checkbox-group {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.filter-section .el-checkbox {
  margin-right: 0;
  margin-bottom: 5px;
}

.cross-info {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.info-tables-container {
  display: flex;
  gap: 20px;
  margin-top: 20px;
  margin-bottom: 20px;
}

.lane-info-table {
  flex: 2;
  flex-shrink: 0;
  background: white;
  border-radius: 8px;
  padding: 5px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.1);
  overflow: hidden;
}

.lane-info-table .el-table {
  width: 100%;
}

.lane-info-table .el-table__body-wrapper {
  overflow-x: auto;
}

.ped-info-table {
  flex: 1;
  flex-shrink: 0;
  background: white;
  border-radius: 8px;
  padding: 5px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.1);
  min-width: 300px;
}

.cross-map-container {
  display: flex;
  gap: 20px;
  margin-top: 20px;
}

.cross-map {
  flex: 2;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.1);
  max-width: 1000px;
  max-height: 1000px;
}

.lamp-status-table {
  flex: 1;
  flex-shrink: 0;
  background: white;
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.1);
  min-width: 300px;
}

.table-header {
  margin-bottom: 16px;
  color: #303133;
  font-size: 16px;
  font-weight: 500;
}

.lamp-status {
  color:white
}

.lamp-status.red {
  background-color: #ff4d4f;
}

.lamp-status.yellow {
  background-color: #faad14;
}

.lamp-status.green {
  background-color: #52c41a;
}

.lamp-status.gray {
  background-color: #666666;
}

.lamp-status.yellow-flash {
  background-color: #faad14;
  animation: flash 1s infinite;
}
.lamp-status.green-flash {
  background-color: #52c41a;
  animation: flash 1s infinite;
}

@keyframes flash {
  0% { opacity: 1; }
  50% { opacity: 0.3; }
  100% { opacity: 1; }
}

.validation-errors {
  margin-bottom: 20px;
}

.validation-errors .el-alert {
  margin-bottom: 10px;
}

.cycle-info {
  margin-top: 20px;
}

.json-dialog {
  max-width: 80vw;
  height: 80vh;
  display: flex;
  flex-direction: column;
}

.json-dialog :deep(.el-message-box__header) {
  padding: 15px 20px;
  border-bottom: 1px solid #eee;
}

.json-dialog :deep(.el-message-box__content) {
  flex: 1;
  overflow: auto;
  padding: 0;
}

.json-dialog :deep(.el-message-box__btns) {
  padding: 10px 20px;
  border-top: 1px solid #eee;
}

.json-dialog :deep(.json-content) {
  position: relative;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.json-dialog :deep(pre) {
  margin: 0;
  white-space: pre-wrap;
  word-wrap: break-word;
  font-family: monospace;
  font-size: 14px;
  line-height: 1.5;
  color: #333;
  background-color: #f5f7fa;
  padding: 15px;
  border-radius: 4px;
  flex: 1;
  overflow: auto;
}
</style>
