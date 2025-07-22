<template>
  <div class="cross-map">
    <div class="lane-legend">
      <div class="legend-title">车道属性图例：</div>
      <div class="legend-items">
        <div class="legend-item"><span class="legend-color lane-attr-0"></span>路口进口</div>
        <div class="legend-item"><span class="legend-color lane-attr-1"></span>路口出口</div>
        <div class="legend-item"><span class="legend-color lane-attr-2"></span>匝道</div>
        <div class="legend-item"><span class="legend-color lane-attr-3"></span>路段车道</div>
        <div class="legend-item"><span class="legend-color lane-attr-9"></span>其他</div>
      </div>
    </div>
    <div class="grid-container">
      <!-- 西北 -->
      <div class="grid-cell northwest">
      </div>
      <!-- 北进口 -->
      <div class="grid-cell north">
        <div class="direction-label">{{ directionDict['1'] }}</div>
        <div class="lanes-container">
          <div class="lanes-split">
            <div class="lanes-left">
              <div v-for="lane in northInLanes" :key="lane.laneNo" :class="['lane', `lane-attr-${lane.attr}`]"
                   :style="getLaneStyle(lane)">
                <div class="lane-info">
                  <div class="lane-angle">{{lane.angle}}°</div>
                  <div class="lane-movement">{{ getMovementText(lane.turn) }}</div>
                  <div class="lane-number">{{lane.laneNo}}</div>
                </div>
                <div v-if="getLaneLampStatus(lane)" class="remain-time">
                  {{ getLaneLampStatus(lane).remainTime }}s
                </div>
              </div>
            </div>
            <div class="lanes-right">
              <div v-for="lane in northOutLanes" :key="lane.laneNo" :class="['lane', `lane-attr-${lane.attr}`]"
                   :style="getLaneStyle(lane)">
                <div class="lane-info">
                  <div class="lane-angle">{{lane.angle}}°</div>
                  <div class="lane-movement">{{ getMovementText(lane.turn) }}</div>
                  <div class="lane-number">{{lane.laneNo}}</div>
                </div>
                <div v-if="getLaneLampStatus(lane)" class="remain-time">
                  {{ getLaneLampStatus(lane).remainTime }}s
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <!-- 东北 -->
      <div class="grid-cell northeast">
      </div>
      <!-- 西进口 -->
      <div class="grid-cell west">
        <div class="direction-label">{{ directionDict['7'] }}</div>
        <div class="lanes-container">
          <div class="lanes-split">
            <div class="lanes-top">
              <div v-for="lane in westOutLanes" :key="lane.laneNo" :class="['lane', `lane-attr-${lane.attr}`]"
                   :style="getLaneStyle(lane)">
                {{lane.angle}}° {{ getMovementText(lane.turn) }} {{lane.laneNo}}
                <div v-if="getLaneLampStatus(lane)" class="remain-time">
                  {{ getLaneLampStatus(lane).remainTime }}s
                </div>
              </div>
            </div>
            <div class="lanes-bottom">
              <div v-for="lane in westInLanes" :key="lane.laneNo" :class="['lane', `lane-attr-${lane.attr}`]"
                   :style="getLaneStyle(lane)">
                {{lane.angle}}° {{ getMovementText(lane.turn) }} {{lane.laneNo}}
                <div v-if="getLaneLampStatus(lane)" class="remain-time">
                  {{ getLaneLampStatus(lane).remainTime }}s
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <!-- 中心区域 -->
      <div class="grid-cell center">
        <div class="cross-name">{{ crossData?.crossName }}</div>
      </div>
      <!-- 东进口 -->
      <div class="grid-cell east">
        <div class="direction-label">{{ directionDict['3'] }}</div>
        <div class="lanes-container">
          <div class="lanes-split">
            <div class="lanes-top">
              <div v-for="lane in eastInLanes" :key="lane.laneNo" :class="['lane', `lane-attr-${lane.attr}`]"
                   :style="getLaneStyle(lane)">
                {{lane.laneNo}} {{ getMovementText(lane.turn) }} {{lane.angle}}°
                <div v-if="getLaneLampStatus(lane)" class="remain-time">
                  {{ getLaneLampStatus(lane).remainTime }}s
                </div>
              </div>
            </div>
            <div class="lanes-bottom">
              <div v-for="lane in eastOutLanes" :key="lane.laneNo" :class="['lane', `lane-attr-${lane.attr}`]"
                   :style="getLaneStyle(lane)">
                {{lane.laneNo}} {{ getMovementText(lane.turn) }} {{lane.angle}}°
                <div v-if="getLaneLampStatus(lane)" class="remain-time">
                  {{ getLaneLampStatus(lane).remainTime }}s
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <!-- 西南 -->
      <div class="grid-cell southwest">
      </div>
      <!-- 南进口 -->
      <div class="grid-cell south">
        <div class="direction-label">{{ directionDict['5'] }}</div>
        <div class="lanes-container">
          <div class="lanes-split">
            <div class="lanes-left">
              <div v-for="lane in southOutLanes" :key="lane.laneNo" :class="['lane', `lane-attr-${lane.attr}`]"
                   :style="getLaneStyle(lane)">
                <div class="lane-info">
                  <div class="lane-number">{{lane.laneNo}}</div>
                  <div class="lane-movement">{{ getMovementText(lane.turn) }}</div>
                  <div class="lane-angle">{{lane.angle}}°</div>
                </div>
                <div v-if="getLaneLampStatus(lane)" class="remain-time">
                  {{ getLaneLampStatus(lane).remainTime }}s
                </div>
              </div>
            </div>
            <div class="lanes-right">
              <div v-for="lane in southInLanes" :key="lane.laneNo" :class="['lane', `lane-attr-${lane.attr}`]"
                   :style="getLaneStyle(lane)">
                <div class="lane-info">
                  <div class="lane-number">{{lane.laneNo}}</div>
                  <div class="lane-movement">{{ getMovementText(lane.turn) }}</div>
                  <div class="lane-angle">{{lane.angle}}°</div>
                </div>
                <div v-if="getLaneLampStatus(lane)" class="remain-time">
                  {{ getLaneLampStatus(lane).remainTime }}s
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <!-- 东南 -->
      <div class="grid-cell southeast">
      </div>
    </div>
    <!-- 车道属性图例 -->
  
  </div>
</template>

<script>
import { computed } from 'vue'

export default {
  name: 'CrossMap',
  props: {
    crossData: {
      type: Object,
      required: true
    }
  },
  setup(props) {
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

    const getMovementText = (turn) => {
      switch (turn) {
        case 11: return '直'
        case 12: return '左'
        case 13: return '右'
        case 21: return '直左'
        case 22: return '直右'
        case 23: return '左右'
        case 24: return '直左右'
        case 31: return '掉'
        case 32: return '掉左'
        case 33: return '掉直'
        case 34: return '掉右'
        case 99: return '其他'
        default: return '未知'
      }
    }

    // 获取灯组状态文本
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

    // 获取灯组类型文本
    const getLampTypeText = (type) => {
      const typeDict = {
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
      return typeDict[type] || `未知灯组(${type})`
    }

    // 获取灯组状态颜色
    const getLampStatusColor = (status) => {
      const colorMap = {
        '11': '#666666', // 灭灯
        '21': '#ff4d4f', // 红灯
        '22': '#faad14', // 黄灯
        '23': '#52c41a', // 绿灯
        '42': '#faad14', // 黄闪
        '43': '#52c41a'  // 绿闪
      }
      return colorMap[status] || '#666666'
    }

    // 获取指定方向的灯组状态
    const getLampStatusByDirection = (direction) => {
      console.log('获取方向灯组状态 - 方向:', direction)
      if (!props.crossData?.lampStatusList) {
        console.log('灯组状态列表为空')
        return []
      }
      const filteredLamps = props.crossData.lampStatusList.filter(lamp => {
        console.log('当前灯组controlDir:', lamp.controlDir)
        const diff1 = Math.abs(lamp.controlDir - direction)
        const diff2 = Math.abs(lamp.controlDir - direction - 360)
        const diff3 = Math.abs(lamp.controlDir - direction + 360)
        const isMatch = diff1 < 10 || diff2 < 10 || diff3 < 10
        if (isMatch) {
          console.log('匹配到灯组:', {
            controlDir: lamp.controlDir,
            lampType: lamp.lampType,
            lampStatus: lamp.lampStatus,
            remainTime: lamp.remainTime
          })
        }
        return isMatch
      })
      console.log('过滤后的灯组数量:', filteredLamps.length)
      return filteredLamps
    }

    // 获取车道对应的灯组状态
    const getLaneLampStatus = (lane) => {
      const lamps = getLampStatusByDirection(lane.angle)
      
      // 根据车道转向属性匹配对应的灯组类型
      const getLampTypeByTurn = (turn) => {
        switch (turn) {
          case 11: // 直行
            return ['10', '21'] // 机动车信号灯、直行灯
          case 12: // 左转
            return ['22'] // 左转灯
          case 13: // 右转
            return ['23'] // 右转灯
          case 21: // 直左混行
            return ['10', '21', '22'] // 机动车信号灯、直行灯、左转灯
          case 22: // 直右混行
            return ['10', '21', '23'] // 机动车信号灯、直行灯、右转灯
          case 23: // 左右混行
            return ['22', '23'] // 左转灯、右转灯
          case 24: // 直左右混行
            return ['10', '21', '22', '23'] // 所有机动车灯
          case 31: // 掉头
            return ['80'] // 掉头灯
          case 32: // 掉头加左转
            return ['80', '22'] // 掉头灯、左转灯
          case 33: // 掉头加直行
            return ['80', '10', '21'] // 掉头灯、机动车信号灯、直行灯
          case 34: // 掉头加右转
            return ['80', '23'] // 掉头灯、右转灯
          default:
            return ['10'] // 默认使用机动车信号灯
        }
      }

      // 获取该车道对应的灯组类型列表
      const allowedLampTypes = getLampTypeByTurn(lane.turn)
      
      // 查找匹配的灯组状态
      return lamps.find(lamp => allowedLampTypes.includes(lamp.lampType))
    }

    // 获取车道样式
    const getLaneStyle = (lane) => {
      const lampStatus = getLaneLampStatus(lane)
      console.log(lampStatus)
      
      if (!lampStatus) return {}
      
      return {
        backgroundColor: getLampStatusColor(lampStatus.lampStatus),
        color: '#fff',
        title: `${getLampTypeText(lampStatus.lampType)}: ${getLampStatusText(lampStatus.lampStatus)} (${lampStatus.remainTime}s)`
      }
    }

    const northInLanes = computed(() => {
      return (props.crossData?.lanes?.filter(lane => lane.direction === 1 && lane.attr === 0) || []).slice().sort((a, b) => a.laneNo - b.laneNo)
    })
    const northOutLanes = computed(() => {
      return (props.crossData?.lanes?.filter(lane => lane.direction === 1 && lane.attr === 1) || []).slice().sort((a, b) => a.laneNo - b.laneNo)
    })
    const southInLanes = computed(() => {
      return (props.crossData?.lanes?.filter(lane => lane.direction === 5 && lane.attr === 0) || []).slice().sort((a, b) => a.laneNo - b.laneNo)
    })
    const southOutLanes = computed(() => {
      return (props.crossData?.lanes?.filter(lane => lane.direction === 5 && lane.attr === 1) || []).slice().sort((a, b) => a.laneNo - b.laneNo)
    })

    const eastInLanes = computed(() => {
      return (props.crossData?.lanes?.filter(lane => lane.direction === 3 && lane.attr === 0) || []).slice().sort((a, b) => a.laneNo - b.laneNo)
    })
    const eastOutLanes = computed(() => {
      return (props.crossData?.lanes?.filter(lane => lane.direction === 3 && lane.attr === 1) || []).slice().sort((a, b) => a.laneNo - b.laneNo)
    })
    const westInLanes = computed(() => {
      return (props.crossData?.lanes?.filter(lane => lane.direction === 7 && lane.attr === 0) || []).slice().sort((a, b) => a.laneNo - b.laneNo)
    })
    const westOutLanes = computed(() => {
      return (props.crossData?.lanes?.filter(lane => lane.direction === 7 && lane.attr === 1) || []).slice().sort((a, b) => a.laneNo - b.laneNo)
    })

    return {
      getMovementText,
      getLampStatusText,
      getLampTypeText,
      getLampStatusColor,
      getLampStatusByDirection,
      getLaneStyle,
      getLaneLampStatus,
      northInLanes,
      northOutLanes,
      southInLanes,
      southOutLanes,
      eastInLanes,
      eastOutLanes,
      westInLanes,
      westOutLanes,
      directionDict
    }
  }
}
</script>

<style scoped>
.cross-map {
  width: 100%;
  aspect-ratio: 1;
  background-color: #f5f7fa;
  border-radius: 8px;
}

.grid-container {
  width: 100%;
  height: 100%;
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: repeat(3, 1fr);
  gap: 2px;
  background-color: #e6e6e6;
  border-radius: 4px;
}

.grid-cell {
  background-color: white;
  border-radius: 4px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  position: relative;
  min-height:200px;
  min-width:200px;
}

.direction-label {
  position: absolute;
  top: 5px;
  left: 50%;
  transform: translateX(-50%);
  font-weight: bold;
  color: #666;
  z-index: 2;
}

.lanes-container {
  display: flex;
  width: 100%;
  height: 100%;
}

.lane {
  background-color: #f0f0f0;
  padding: 2px 2px;
  border-radius: 4px;
  font-size: 12px;
  text-align: center;
  color: #666;
  margin: 2px;
  transition: all 0.3s ease;
  position: relative;
}

.lane.exit {
  background-color: #e6e6e6;
  color: #999;
  border: 1px dashed #ccc;
}

.center {
  background-color: #f8f8f8;
  display: flex;
  align-items: center;
  justify-content: center;
}

.cross-name {
  font-size: 14px;
  color: #333;
  text-align: center;
  padding: 10px;
}

/* 旋转方向标签 */
.north .direction-label { transform: rotate(0deg); }
.east .direction-label { transform: rotate(90deg); }
.south .direction-label { transform: rotate(180deg); }
.west .direction-label { transform: rotate(270deg); }

.lanes-split {
  display: flex;
  width: 100%;
  height: 100%;
  gap: 2px;
}

.lanes-left, .lanes-right {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.lanes-top, .lanes-bottom {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.north .lanes-split {
  flex-direction: row;
}

.north .lanes-left {
  flex-direction: row;
  flex-direction: row-reverse;
}
.north .lanes-right {
  flex-direction: row;

}


.south .lanes-split {
  flex-direction: row;
}
.south .lanes-right {
  flex-direction: row;
}

.south .lanes-left {
  flex-direction: row;
  flex-direction: row-reverse;
}

.east .lanes-split {
  flex-direction: column;
}

.east .lanes-top {
  flex-direction: column-reverse;
}

.west .lanes-split {
  flex-direction: column;
}

.west .lanes-top {
  flex-direction: column;
}
.north .lane {
  display: flex;
  align-items: flex-end;
  justify-content: center;
  background-color: #f0f0f0;
  margin: 2px;
  font-size: 12px;
  max-width: 15px;
}

.north .lane-info {

  display: flex;
  flex-direction: column;
  align-items: center;
  white-space: nowrap;
  padding: 2px;
  gap: 4px;
}

.north .lane-angle {
  transform: rotate(-90deg);
  margin-bottom: 10px;
}

.north .lane-movement {
  transform: rotate(-90deg);
  font-weight: bold;
}

.north .lane-number {

  font-size: 12px;
  margin-top: 10px;
}

.south .lane {
  display: flex;
  align-items: flex-start;
  justify-content: center;
  background-color: #f0f0f0;
  margin: 2px;
  font-size: 12px;
  max-width: 15px;
}

.south .lane-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  white-space: nowrap;
  padding: 2px;
}

.south .lane-angle {
  transform: rotate(90deg);
  margin-top :10px;
}

.south .lane-movement {
  transform: rotate(90deg);
  font-weight: bold;
}

.south .lane-number {
  font-size: 12px;
  margin-bottom: 10px;
}

.east .lane {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  background-color: #f0f0f0;
  margin: 2px;
  font-size: 12px;
}

.west .lane {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  background-color: #f0f0f0;
  margin: 2px;
  font-size: 12px;
}

.lamp-status {
  position: absolute;
  top: -15px;
  left: 50%;
  transform: translateX(-50%);
  width: 8px;
  height: 8px;
  border-radius: 50%;
  border: 1px solid rgba(0, 0, 0, 0.1);
}

.lamp-status-text {
  position: absolute;
  top: -25px;
  left: 50%;
  transform: translateX(-50%);
  font-size: 10px;
  white-space: nowrap;
  color: #666;
}

.remain-time {
  position: absolute;
  top: -15px;
  left: 50%;
  transform: translateX(-50%);
  background-color: rgba(0, 0, 0, 0.6);
  color: #fff;
  padding: 2px 4px;
  border-radius: 2px;
  font-size: 10px;
  white-space: nowrap;
}

/* 车道属性样式定义 */
.lane-attr-0 { /* 路口进口 */
  border: 2px solid #0052cc;
  background-color: #cce0ff;
}
.lane-attr-1 { /* 路口出口 */
  border: 2px solid #d32f2f;
  background-color: #ffd6d6;
}
.lane-attr-2 { /* 匝道 */
  border: 2px dashed #ff9800;
  background-color: #fff3cd;
}
.lane-attr-3 { /* 路段车道 */
  border: 2px solid #388e3c;
  background-color: #d0f5e8;
}
.lane-attr-9 { /* 其他 */
  border: 2px dotted #7b1fa2;
  background-color: #f3e5f5;
}

/* 车道属性图例样式 */
.lane-legend {
  margin-top: 16px;
  padding: 10px 16px;
  background: #f8f9fa;
  border-radius: 4px;
  border: 1px solid #dcdfe6;
  width: fit-content;
  font-size: 14px;
}
.legend-title {
  font-weight: bold;
  margin-bottom: 8px;
}
.legend-items {
  display: flex;
  gap: 18px;
}
.legend-item {
  display: flex;
  align-items: center;
  gap: 6px;

}
.legend-color {
  display: inline-block;
  width: 22px;
  height: 18px;
  border-radius: 3px;
  border: 2px solid transparent;
  margin-right: 2px;
}
</style> 