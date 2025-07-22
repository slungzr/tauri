<template>
  <div class="browser-container">
    
    <!-- 左侧键列表 -->
    <div class="key-list">
      <div class="action-buttons">
        <el-button type="primary" @click="goToParameterCheck">
          <el-icon><Check /></el-icon>
          批量参数校验
        </el-button>
        <el-button type="primary" @click="goToLogin">
          <el-icon><Back /></el-icon>
          返回登录
        </el-button>
      </div>
      <div class="search-box">
        <el-input
          v-model="searchPattern"
          placeholder="请输入路口号..."
          @keyup.enter="handleSearch"
          style="font-size: 12px;"
        >
          <template #append>
            <el-button 
              @click="handleSearch" 
              style="font-size: 12px;"
            >
              <el-icon><Search /></el-icon>
            </el-button>
          </template>
        </el-input>
      </div>
      
      <el-scrollbar height="calc(100vh - 60px)">
        <el-tree
          :data="keyTree"
          :props="defaultProps"
          @node-click="handleNodeClick"
          node-key="key"
          highlight-current
        >
          <template #default="{ node, data }">
            <span class="custom-tree-node">
              <el-icon><Document /></el-icon>
              <span>{{ node.label }}</span>
            </span>
          </template>
        </el-tree>
      </el-scrollbar>
    </div>

    <!-- 右侧数据展示 -->
    <div class="data-view">
      <template v-if="selectedKey">
        <div class="key-header">
          <h3>{{ selectedKey }}</h3>
         
          <el-button-group>
           
            <el-button type="primary" @click="refreshValue">
              <el-icon><Refresh /></el-icon>
              刷新
            </el-button>
          </el-button-group>
        
        </div>

        <div class="value-display">
          <el-tabs v-model="activeTab">
            <el-tab-pane label="基本信息" name="basic">
              <div class="view-switch">
                <el-radio-group v-model="basicViewMode" size="small">
                  <el-radio-button label="table">表格视图</el-radio-button>
                  <el-radio-button label="json">JSON视图</el-radio-button>
                </el-radio-group>
              </div>

              <!-- 表格视图 -->
              <template v-if="basicViewMode === 'table'">
                <el-descriptions :column="1" border>
                  <el-descriptions-item label="路口编号 (CrossID)">
                    <div class="field-description">全局唯一，取值区域编号+5位数字</div>
                    <div v-html="formatValue(value?.crossID, 'crossID')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="路口名称 (CrossName)">
                    <div class="field-description">最大长度100</div>
                    <div v-html="formatValue(value?.crossName, 'crossName')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="路口形状 (Feature)">
                    <div class="field-description">
                      取值：<br>
                      10：行人过街<br>
                      12：2次行人过街<br>
                      23：T形、Y形<br>
                      24：十字形<br>
                      35：五岔路口<br>
                      36：六岔路口<br>
                      39：多岔路口<br>
                      40：环形交叉口(环岛)<br>
                      50：匝道<br>
                      51：匝道-入口<br>
                      52：匝道-出口<br>
                      61：快速路主路路段(交汇区)<br>
                      90：其他
                    </div>
                    <div v-html="formatValue(value?.feature, 'feature')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="路口等级 (Grade)">
                    <div class="field-description">
                      取值：<br>
                      11：一级，主干路与主干路相交交叉口<br>
                      12：二级，主干路与次干路相交交叉口<br>
                      13：三级，主干路与支路相交交叉口<br>
                      21：四级，次干路与次干路相交交叉口<br>
                      22：五级，次干路与支路相交交叉口<br>
                      31：六级，支路与支路相交交叉口<br>
                      99：其他
                    </div>
                    <div v-html="formatValue(value?.grade, 'grade')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="检测器序号列表 (DetNoList)">
                    <div class="field-description">无检测器时可空</div>
                    <div v-html="formatValue(value?.detNoList, 'detNoList')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="车道序号列表 (LaneNoList)">
                    <div class="field-description">包含至少1个车道序号</div>
                    <div v-html="formatValue(value?.laneNoList, 'laneNoList')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="人行横道序号列表 (PedestrianNoList)">
                    <div class="field-description">可空</div>
                    <div v-html="formatValue(value?.pedestrianNoList, 'pedestrianNoList')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="信号灯组序号列表 (LampGroupNoList)">
                    <div class="field-description">包含至少1个灯组序号</div>
                    <div v-html="formatValue(value?.lampGroupNoList, 'lampGroupNoList')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="信号组序号列表 (SignalGroupNoList)">
                    <div class="field-description">包含至少1个信号组序号</div>
                    <div v-html="formatValue(value?.signalGroupNoList, 'signalGroupNoList')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="绿冲突矩阵 (GreenConflictMatrix)">
                    <div class="field-description">
                      由0、1组成的字符串，长度为信号组数量的平方，按照矩阵自左向右、自上而下排列。<br>
                      每位字符取值：<br>
                      0：不冲突<br>
                      1：冲突
                    </div>
                    <div v-html="formatValue(value?.greenConflictMatrix, 'greenConflictMatrix')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="阶段号列表 (StageNoList)">
                    <div class="field-description">包含至少1个阶段号</div>
                    <div v-html="formatValue(value?.stageNoList, 'stageNoList')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="配时方案序号列表 (PlanNoList)">
                    <div class="field-description">包含至少1个配时方案序号</div>
                    <div v-html="formatValue(value?.planNoList, 'planNoList')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="日计划号列表 (DayPlanNoList)">
                    <div class="field-description">包含至少1个日计划号</div>
                    <div v-html="formatValue(value?.dayPlanNoList, 'dayPlanNoList')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="调度号列表 (ScheduleNoList)">
                    <div class="field-description">包含至少1个调度号</div>
                    <div v-html="formatValue(value?.scheduleNoList, 'scheduleNoList')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="路口中心位置经度 (Longitude)">
                    <div class="field-description">使用WGS84坐标系，符合GA/T 543.9—2016的DE01119</div>
                    <div v-html="formatValue(value?.longitude, 'longitude')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="路口中心位置纬度 (Latitude)">
                    <div class="field-description">使用WGS84坐标系，符合GA/T 543.9—2016的DE01120</div>
                    <div v-html="formatValue(value?.latitude, 'latitude')"></div>
                  </el-descriptions-item>
                  <el-descriptions-item label="路口位置海拔高度 (Altitude)">
                    <div class="field-description">可空，单位为米（m）</div>
                    <div v-html="formatValue(value?.altitude, 'altitude')"></div>
                  </el-descriptions-item>
                </el-descriptions>
              </template>

              <!-- JSON视图 -->
              <template v-else>
                <div class="json-view">
                  <div class="json-header">
                    <span class="json-title">原始JSON数据</span>
                    <el-button type="primary" size="small" @click="copyJson('signalGroups')">
                      <el-icon><Document /></el-icon>
                      复制
                    </el-button>
                  </div>
                  <pre class="json-content">{{ getRawJsonData('signalGroups') }}</pre>
                </div>
              </template>
            </el-tab-pane>
            
            <el-tab-pane :label="`灯组信息 (${lampGroupNos.length})`" name="lampGroups">
              <div class="view-switch">
                <el-radio-group v-model="detailViewModes.lampGroups" size="small">
                  <el-radio-button label="table">表格视图</el-radio-button>
                  <el-radio-button label="json">JSON视图</el-radio-button>
                </el-radio-group>
              </div>

              <!-- 表格视图 -->
              <template v-if="detailViewModes.lampGroups === 'table'">
                <div class="filter-bar">
                  <div class="column-select">
                    <el-radio-group v-model="detailColumns.lampGroups" size="small">
                      <el-radio-button label="2">2列</el-radio-button>
                      <el-radio-button label="3">3列</el-radio-button>
                      <el-radio-button label="4">4列</el-radio-button>
                      <el-radio-button label="6">6列</el-radio-button>
                    </el-radio-group>
                  </div>
                </div>
                <div class="detail-grid" :class="`columns-${detailColumns.lampGroups}`">
                  <el-card v-for="no in lampGroupNos" 
                    :key="no" 
                    class="detail-card"
                  >
                    <template #header>
                      <div class="card-header">
                        <span>灯组 {{ no }}</span>
                        <el-tag size="small" type="info">1049Cache:param:LampGroup:{{ value?.crossID }}:{{ no }}</el-tag>
                      </div>
                    </template>
                    <el-descriptions :column="1" border>
                      <template v-for="(val, key) in lampGroupDetails[no]" :key="key">
                        <el-descriptions-item v-if="key !== 'status'" :label="formatLabel(key)">
                          <div v-html="formatValue(val, key, 'lampGroup')"></div>
                        </el-descriptions-item>
                      </template>
                    </el-descriptions>
                  </el-card>
                </div>
              </template>

              <!-- JSON视图 -->
              <template v-else>
                <div class="json-view">
                  <div class="json-header">
                    <span class="json-title">原始JSON数据</span>
                    <el-button type="primary" size="small" @click="copyJson('lampGroups')">
                      <el-icon><Document /></el-icon>
                      复制
                    </el-button>
                  </div>
                  <pre class="json-content">{{ getRawJsonData('lampGroups') }}</pre>
                </div>
              </template>
            </el-tab-pane>

            <el-tab-pane :label="`车道信息 (${laneNos.length})`" name="lanes">
              <div class="view-switch">
                <el-radio-group v-model="detailViewModes.lanes" size="small">
                  <el-radio-button label="table">表格视图</el-radio-button>
                  <el-radio-button label="json">JSON视图</el-radio-button>
                </el-radio-group>
              </div>

              <!-- 表格视图 -->
              <template v-if="detailViewModes.lanes === 'table'">
                <div class="filter-bar">
                  <div class="column-select">
                    <el-radio-group v-model="detailColumns.lanes" size="small">
                      <el-radio-button label="2">2列</el-radio-button>
                      <el-radio-button label="3">3列</el-radio-button>
                      <el-radio-button label="4">4列</el-radio-button>
                      <el-radio-button label="6">6列</el-radio-button>
                    </el-radio-group>
                  </div>
                </div>
                <div class="detail-grid" :class="`columns-${detailColumns.lanes}`">
                  <el-card v-for="no in laneNos" 
                    :key="no" 
                    class="detail-card"
                  >
                    <template #header>
                      <div class="card-header">
                        <span>车道 {{ no }}</span>
                        <el-tag size="small" type="info">1049Cache:param:LaneParam:{{ value?.crossID }}:{{ no }}</el-tag>
                      </div>
                    </template>
                    <el-descriptions :column="1" border>
                      <template v-for="(val, key) in laneDetails[no]" :key="key">
                        <el-descriptions-item :label="formatLabel(key)">
                          <div v-html="formatValue(val, key, 'lane')"></div>
                        </el-descriptions-item>
                      </template>
                    </el-descriptions>
                  </el-card>
                </div>
              </template>

              <!-- JSON视图 -->
              <template v-else>
                <div class="json-view">
                  <div class="json-header">
                    <span class="json-title">原始JSON数据</span>
                    <el-button type="primary" size="small" @click="copyJson('lanes')">
                      <el-icon><Document /></el-icon>
                      复制
                    </el-button>
                  </div>
                  <pre class="json-content">{{ getRawJsonData('lanes') }}</pre>
                </div>
              </template>
            </el-tab-pane>

            <el-tab-pane :label="`人行横道参数 (${pedestrianNos.length})`" name="pedestrians">
              <div class="view-switch">
                <el-radio-group v-model="detailViewModes.pedestrians" size="small">
                  <el-radio-button label="table">表格视图</el-radio-button>
                  <el-radio-button label="json">JSON视图</el-radio-button>
                </el-radio-group>
              </div>

              <!-- 表格视图 -->
              <template v-if="detailViewModes.pedestrians === 'table'">
                <div class="filter-bar">
                  <div class="column-select">
                    <el-radio-group v-model="detailColumns.pedestrians" size="small">
                      <el-radio-button label="2">2列</el-radio-button>
                      <el-radio-button label="3">3列</el-radio-button>
                      <el-radio-button label="4">4列</el-radio-button>
                      <el-radio-button label="6">6列</el-radio-button>
                    </el-radio-group>
                  </div>
                </div>
                <div class="detail-grid" :class="`columns-${detailColumns.pedestrians}`">
                  <el-card v-for="no in pedestrianNos" 
                    :key="no" 
                    class="detail-card"
                  >
                    <template #header>
                      <div class="card-header">
                        <span>行人 {{ no }}</span>
                        <el-tag size="small" type="info">1049Cache:param:PedestrianParam:{{ value?.crossID }}:{{ no }}</el-tag>
                      </div>
                    </template>
                    <el-descriptions :column="1" border>
                      <template v-for="(val, key) in pedestrianDetails[no]" :key="key">
                        <el-descriptions-item :label="formatLabel(key)">
                          <div v-html="formatValue(val, key, 'pedestrian')"></div>
                        </el-descriptions-item>
                      </template>
                    </el-descriptions>
                  </el-card>
                </div>
              </template>

              <!-- JSON视图 -->
              <template v-else>
                <div class="json-view">
                  <div class="json-header">
                    <span class="json-title">原始JSON数据</span>
                    <el-button type="primary" size="small" @click="copyJson('pedestrians')">
                      <el-icon><Document /></el-icon>
                      复制
                    </el-button>
                  </div>
                  <pre class="json-content">{{ getRawJsonData('pedestrians') }}</pre>
                </div>
              </template>
            </el-tab-pane>

            <el-tab-pane :label="`相位信息 (${stageNos.length})`" name="stages">
              <div class="view-switch">
                <el-radio-group v-model="detailViewModes.stages" size="small">
                  <el-radio-button label="table">表格视图</el-radio-button>
                  <el-radio-button label="json">JSON视图</el-radio-button>
                </el-radio-group>
              </div>

              <!-- 表格视图 -->
              <template v-if="detailViewModes.stages === 'table'">
                <div class="filter-bar">
                  <div class="column-select">
                    <el-radio-group v-model="detailColumns.stages" size="small">
                      <el-radio-button label="2">2列</el-radio-button>
                      <el-radio-button label="3">3列</el-radio-button>
                      <el-radio-button label="4">4列</el-radio-button>
                      <el-radio-button label="6">6列</el-radio-button>
                    </el-radio-group>
                  </div>
                </div>
                <div class="detail-grid" :class="`columns-${detailColumns.stages}`">
                  <el-card v-for="no in stageNos" 
                    :key="no" 
                    class="detail-card"
                  >
                    <template #header>
                      <div class="card-header">
                        <span>相位 {{ no }}</span>
                        <el-tag size="small" type="info">1049Cache:param:StageParam:{{ value?.crossID }}:{{ no }}</el-tag>
                      </div>
                    </template>
                    <el-descriptions :column="1" border>
                      <template v-for="(val, key) in stageDetails[no]" :key="key">
                        <el-descriptions-item :label="formatLabel(key)">
                          <template v-if="key === 'signalGroupStatusList'">
                            <div v-for="status in val.signalGroupStatus" :key="status.signalGroupNo">
                              <div class="signal-group-status">
                                <strong>信号组 {{ status.signalGroupNo }}：</strong>
                                <div v-html="formatStageValue(status.lampStatus, 'lampStatus', status)"></div>
                              </div>
                            </div>
                          </template>
                          <template v-if="key === 'phaseInfo'">
                           <phase :phaseImage="val"></phase>
                          </template>
                          <template v-else>
                            <div v-html="formatValue(val, key, 'stage')"></div>
                          </template>
                        </el-descriptions-item>
                      </template>
                    </el-descriptions>
                  
                  </el-card>
                </div>
              </template>

              <!-- JSON视图 -->
              <template v-else>
                <div class="json-view">
                  <div class="json-header">
                    <span class="json-title">原始JSON数据</span>
                    <el-button type="primary" size="small" @click="copyJson('stages')">
                      <el-icon><Document /></el-icon>
                      复制
                    </el-button>
                  </div>
                  <pre class="json-content">{{ getRawJsonData('stages') }}</pre>
                </div>
              </template>
            </el-tab-pane>

            <el-tab-pane :label="`检测器信息 (${detNos.length})`" name="detectors">
              <div class="view-switch">
                <el-radio-group v-model="detailViewModes.detectors" size="small">
                  <el-radio-button label="table">表格视图</el-radio-button>
                  <el-radio-button label="json">JSON视图</el-radio-button>
                </el-radio-group>
              </div>

              <!-- 表格视图 -->
              <template v-if="detailViewModes.detectors === 'table'">
                <div class="filter-bar">
                  <div class="column-select">
                    <el-radio-group v-model="detailColumns.detectors" size="small">
                      <el-radio-button label="2">2列</el-radio-button>
                      <el-radio-button label="3">3列</el-radio-button>
                      <el-radio-button label="4">4列</el-radio-button>
                      <el-radio-button label="6">6列</el-radio-button>
                    </el-radio-group>
                  </div>
                </div>
                <div class="detail-grid" :class="`columns-${detailColumns.detectors}`">
                  <el-card v-for="no in detNos" 
                    :key="no" 
                    class="detail-card"
                  >
                    <template #header>
                      <div class="card-header">
                        <span>检测器 {{ no }}</span>
                        <el-tag size="small" type="info">1049Cache:param:DetParam:{{ value?.crossID }}:{{ no }}</el-tag>
                      </div>
                    </template>
                    <el-descriptions :column="1" border>
                      <template v-for="(val, key) in detDetails[no]" :key="key">
                        <el-descriptions-item :label="formatLabel(key)">
                          <div v-html="formatValue(val, key, 'detector')"></div>
                        </el-descriptions-item>
                      </template>
                    </el-descriptions>
                  </el-card>
                </div>
              </template>

              <!-- JSON视图 -->
              <template v-else>
                <div class="json-view">
                  <div class="json-header">
                    <span class="json-title">原始JSON数据</span>
                    <el-button type="primary" size="small" @click="copyJson('detectors')">
                      <el-icon><Document /></el-icon>
                      复制
                    </el-button>
                  </div>
                  <pre class="json-content">{{ getRawJsonData('detectors') }}</pre>
                </div>
              </template>
            </el-tab-pane>

            <el-tab-pane :label="`方案信息 (${planNos.length})`" name="plans">
              <div class="view-switch">
                <el-radio-group v-model="detailViewModes.plans" size="small">
                  <el-radio-button label="table">表格视图</el-radio-button>
                  <el-radio-button label="json">JSON视图</el-radio-button>
                </el-radio-group>
              </div>

              <!-- 表格视图 -->
              <template v-if="detailViewModes.plans === 'table'">
                <div class="filter-bar">
                  <div class="column-select">
                    <el-radio-group v-model="detailColumns.plans" size="small">
                      <el-radio-button label="2">2列</el-radio-button>
                      <el-radio-button label="3">3列</el-radio-button>
                      <el-radio-button label="4">4列</el-radio-button>
                      <el-radio-button label="6">6列</el-radio-button>
                    </el-radio-group>
                  </div>
                </div>
                <div class="detail-grid" :class="`columns-${detailColumns.plans}`">
                  <el-card v-for="no in planNos" 
                    :key="no" 
                    class="detail-card"
                  >
                    <template #header>
                      <div class="card-header">
                        <span>方案 {{ no }}</span>
                        <el-tag size="small" type="info">1049Cache:param:PlanParam:{{ value?.crossID }}:{{ no }}</el-tag>
                      </div>
                    </template>
                    <el-descriptions :column="1" border>
                      <template v-for="(val, key) in planDetails[no]" :key="key">
                        <el-descriptions-item :label="formatLabel(key)">
                          <div v-html="formatValue(val, key, 'plan')"></div>
                        </el-descriptions-item>
                      </template>
                    </el-descriptions>
                  </el-card>
                </div>
              </template>

              <!-- JSON视图 -->
              <template v-else>
                <div class="json-view">
                  <div class="json-header">
                    <span class="json-title">原始JSON数据</span>
                    <el-button type="primary" size="small" @click="copyJson('plans')">
                      <el-icon><Document /></el-icon>
                      复制
                    </el-button>
                  </div>
                  <pre class="json-content">{{ getRawJsonData('plans') }}</pre>
                </div>
              </template>
            </el-tab-pane>

            <el-tab-pane :label="`信号组信息 (${signalGroupNos.length})`" name="signalGroups">
              <div class="view-switch">
                <el-radio-group v-model="detailViewModes.signalGroups" size="small">
                  <el-radio-button label="table">表格视图</el-radio-button>
                  <el-radio-button label="json">JSON视图</el-radio-button>
                </el-radio-group>
              </div>

              <!-- 表格视图 -->
              <template v-if="detailViewModes.signalGroups === 'table'">
                <div class="filter-bar">
                  <div class="column-select">
                    <el-radio-group v-model="detailColumns.signalGroups" size="small">
                      <el-radio-button label="2">2列</el-radio-button>
                      <el-radio-button label="3">3列</el-radio-button>
                      <el-radio-button label="4">4列</el-radio-button>
                      <el-radio-button label="6">6列</el-radio-button>
                    </el-radio-group>
                  </div>
                </div>
                <div class="detail-grid" :class="`columns-${detailColumns.signalGroups}`">
                  <el-card v-for="no in signalGroupNos" 
                    :key="no" 
                    class="detail-card"
                  >
                    <template #header>
                      <div class="card-header">
                        <span>信号组 {{ no }}</span>
                        <el-tag size="small" type="info">1049Cache:param:SignalGroupParam:{{ value?.crossID }}:{{ no }}</el-tag>
                      </div>
                    </template>
                    <el-descriptions :column="1" border>
                      <template v-for="(val, key) in signalGroupDetails[no]" :key="key">
                        <el-descriptions-item :label="formatLabel(key)">
                          <div v-html="formatValue(val, key, 'signalGroup')"></div>
                        </el-descriptions-item>
                      </template>
                    </el-descriptions>
                  </el-card>
                </div>
              </template>

              <!-- JSON视图 -->
              <template v-else>
                <div class="json-view">
                  <div class="json-header">
                    <span class="json-title">原始JSON数据</span>
                    <el-button type="primary" size="small" @click="copyJson('signalGroups')">
                      <el-icon><Document /></el-icon>
                      复制
                    </el-button>
                  </div>
                  <pre class="json-content">{{ getRawJsonData('signalGroups') }}</pre>
                </div>
              </template>
            </el-tab-pane>
            <el-tab-pane :label="`日计划信息 (${dayPlanNos.length})`" name="dayPlans">
              <div class="view-switch">
                <el-radio-group v-model="detailViewModes.dayPlans" size="small">
                  <el-radio-button label="table">表格视图</el-radio-button>
                  <el-radio-button label="json">JSON视图</el-radio-button>
                </el-radio-group>
              </div>

              <!-- 表格视图 -->
              <template v-if="detailViewModes.dayPlans === 'table'">
                <div class="filter-bar">
                  <div class="column-select">
                    <el-radio-group v-model="detailColumns.dayPlans" size="small">
                      <el-radio-button label="2">2列</el-radio-button>
                      <el-radio-button label="3">3列</el-radio-button>
                      <el-radio-button label="4">4列</el-radio-button>
                      <el-radio-button label="6">6列</el-radio-button>
                    </el-radio-group>
                  </div>
                </div>
                <div class="detail-grid" :class="`columns-${detailColumns.dayPlans}`">
                  <el-card v-for="no in dayPlanNos" 
                    :key="no" 
                    class="detail-card"
                  >
                    <template #header>
                      <div class="card-header">
                        <span>日计划 {{ no }}</span>
                        <el-tag size="small" type="info">1049Cache:param:DayPlanParam:{{ value?.crossID }}:{{ no }}</el-tag>
                      </div>
                    </template>
                    <el-descriptions :column="1" border>
                      <el-descriptions-item label="路口编号 (CrossID)">
                        <div class="field-description">全局唯一，取值区域编号+6位数字</div>
                        <div v-html="formatValue(dayPlanDetails[no]?.crossID, 'crossID', dayPlanDetails[no])"></div>
                      </el-descriptions-item>
                      <el-descriptions-item label="日计划号 (DayPlanNo)">
                        <div class="field-description">从1开始顺序取值，范围1-99999</div>
                        <div v-html="formatValue(dayPlanDetails[no]?.dayPlanNo, 'dayPlanNo', dayPlanDetails[no])"></div>
                      </el-descriptions-item>
                      <el-descriptions-item label="时段信息列表 (PeriodList)">
                        <div class="field-description">包含至少一个时段，每个时段包含开始时间、配时方案号和控制方式</div>
                        <el-table :data="dayPlanDetails[no]?.periodList?.period" border size="small">
                          <el-table-column prop="startTime" label="开始时间" width="120">
                            <template #default="{ row }">
                              {{ row.startTime }}
                            </template>
                          </el-table-column>
                          <el-table-column prop="planNo" label="配时方案号" width="120">
                            <template #default="{ row }">
                              {{ row.planNo }}
                            </template>
                          </el-table-column>
                          <el-table-column prop="ctrlMode" label="控制方式" width="180">
                            <template #default="{ row }">
                                <div v-html="formatValue(row.ctrlMode, 'ctrlMode')"></div>
                            </template>
                          </el-table-column>
                        </el-table>
                      </el-descriptions-item>
                    </el-descriptions>
                  </el-card>
                </div>
              </template>

              <!-- JSON视图 -->
              <template v-else>
                <div class="json-view">
                  <div class="json-header">
                    <span class="json-title">原始JSON数据</span>
                    <el-button type="primary" size="small" @click="copyJson('dayPlans')">
                      <el-icon><Document /></el-icon>
                      复制
                    </el-button>
                  </div>
                  <pre class="json-content">{{ getRawJsonData('dayPlans') }}</pre>
                </div>
              </template>
            </el-tab-pane>

            <el-tab-pane :label="`调度信息 (${scheduleNos.length})`" name="schedules">
              <div class="view-switch">
                <el-radio-group v-model="detailViewModes.schedules" size="small">
                  <el-radio-button label="table">表格视图</el-radio-button>
                  <el-radio-button label="json">JSON视图</el-radio-button>
                </el-radio-group>
              </div>

              <!-- 表格视图 -->
              <template v-if="detailViewModes.schedules === 'table'">
                <div class="filter-bar">
                  <el-select v-model="scheduleTypeFilter" placeholder="按调度类型过滤" clearable>
                    <el-option label="全部" value="all" />
                    <el-option label="特殊日调度" value="1" />
                    <el-option label="时间段周调度" value="2" />
                    <el-option label="周调度" value="3" />
                  </el-select>
                  <span class="filter-count" v-if="scheduleTypeFilter !== 'all'">
                    已过滤: {{ filteredScheduleNos.length }}/{{ scheduleNos.length }}
                  </span>
                  <div class="column-select">
                    <el-radio-group v-model="detailColumns.schedules" size="small">
                      <el-radio-button label="2">2列</el-radio-button>
                      <el-radio-button label="3">3列</el-radio-button>
                      <el-radio-button label="4">4列</el-radio-button>
                      <el-radio-button label="6">6列</el-radio-button>
                    </el-radio-group>
                  </div>
                </div>
                <div class="detail-grid" :class="`columns-${detailColumns.schedules}`">
                  <el-card v-for="no in filteredScheduleNos" 
                    :key="no" 
                    class="detail-card"
                  >
                    <!-- 保持原有的卡片内容不变 -->
                    <template #header>
                      <div class="card-header">
                        <span>调度 {{ no }}</span>
                        <el-tag size="small" type="info">1049Cache:param:ScheduleParam:{{ value?.crossID }}:{{ no }}</el-tag>
                      </div>
                    </template>
                    <el-descriptions :column="1" border>
                      <el-descriptions-item label="路口编号 (CrossID)">
                        <div class="field-description">全局唯一，取值区域编号+5位数字</div>
                        <div v-html="formatValue(scheduleDetails[no]?.crossID, 'crossID', scheduleDetails[no])"></div>
                      </el-descriptions-item>
                      <el-descriptions-item label="调度号 (ScheduleNo)">
                        <div class="field-description">从1开始顺序取值，范围1-999</div>
                        <div v-html="formatValue(scheduleDetails[no]?.scheduleNo, 'scheduleNo', scheduleDetails[no])"></div>
                      </el-descriptions-item>
                      <el-descriptions-item label="调度类型 (Type)">
                        <div class="field-description">
                          调度类型的优先级由高到低，分别取值：<br>
                          1：特殊日调度（由StartDay到EndDay标识的1天或多天）<br>
                          2：时间段周调度（StartDay到EndDay中的周几）<br>
                          3：周调度
                        </div>
                        <div v-html="formatValue(scheduleDetails[no]?.type, 'type', { type: scheduleDetails[no]?.type })"></div>
                      </el-descriptions-item>
                      <el-descriptions-item label="开始月日 (StartDay)">
                        <div class="field-description">
                          格式为月月-日日（MM-DD）<br>
                          注：调度类型为3（周调度）时无意义
                        </div>
                        <div v-html="formatValue(scheduleDetails[no]?.startDay, 'startDay', scheduleDetails[no])"></div>
                      </el-descriptions-item>
                      <el-descriptions-item label="结束月日 (EndDay)">
                        <div class="field-description">
                          格式为月月-日日（MM-DD）<br>
                          注：调度类型为3（周调度）时无意义
                        </div>
                        <div v-html="formatValue(scheduleDetails[no]?.endDay, 'endDay', scheduleDetails[no])"></div>
                      </el-descriptions-item>
                      <el-descriptions-item label="周几 (WeekDay)">
                        <div class="field-description">
                          调度类型为1时无意义；<br>
                          调度类型为2或3时，取值为1-7分别代表周一至周日
                        </div>
                        <div v-html="formatValue(scheduleDetails[no]?.weekDay, 'weekDay', scheduleDetails[no])"></div>
                      </el-descriptions-item>
                      <el-descriptions-item label="日计划号 (DayPlanNo)">
                        <div class="field-description">日计划号</div>
                        <div v-html="formatValue(scheduleDetails[no]?.dayPlanNo, 'dayPlanNo', scheduleDetails[no])"></div>
                      </el-descriptions-item>
                    </el-descriptions>
                  </el-card>
                </div>
              </template>

              <!-- JSON视图 -->
              <template v-else>
                <div class="filter-bar">
                  <el-select v-model="scheduleTypeFilter" placeholder="按调度类型过滤" clearable>
                    <el-option label="全部" value="all" />
                    <el-option label="特殊日调度" value="1" />
                    <el-option label="时间段周调度" value="2" />
                    <el-option label="周调度" value="3" />
                  </el-select>
                  <span class="filter-count" v-if="scheduleTypeFilter !== 'all'">
                    已过滤: {{ filteredScheduleNos.length }}/{{ scheduleNos.length }}
                  </span>
                </div>
                <div class="json-view">
                  <div class="json-header">
                    <span class="json-title">原始JSON数据</span>
                    <el-button type="primary" size="small" @click="copyJson('schedules')">
                      <el-icon><Document /></el-icon>
                      复制
                    </el-button>
                  </div>
                  <pre class="json-content">{{ getFilteredRawJsonData('schedules') }}</pre>
                </div>
              </template>
            </el-tab-pane>

            <!-- 添加路口图标签页 -->
            <el-tab-pane label="路口图" name="crossMap">
              <!-- 运行信息表格 -->
              <div v-if="runInfo" style="margin-bottom: 20px;">
                <h3>控制方式</h3>
                <el-table :data="[runInfo.crossCtrlInfo]" border style="width: 100%">
                  <el-table-column prop="crossID" label="路口编号" />
                  <el-table-column prop="controlMode" label="控制模式" />
                  <el-table-column prop="planNo" label="当前方案号" />
                  <el-table-column prop="time" label="切换时间" />
                  <el-table-column label="原始信息">
                    <template #default="scope">
                      {{ JSON.stringify(scope.row) }}
                    </template>
                  </el-table-column>
                </el-table>
                <h3>当前方案</h3>
                <template v-if="runInfo.planInfo">
                  <el-table :data="[runInfo.planInfo]" border style="width: 100%">
                    <el-table-column prop="planNo" label="方案号" />
                    <el-table-column prop="planName" label="方案名称" />
                    <el-table-column prop="cycleLen" label="周期长度">
                      <template #default="scope">
                        {{ scope.row?.cycleLen }}秒
                      </template>
                    </el-table-column>
                    <el-table-column prop="offset" label="相位差" />
                    <el-table-column prop="coordStageNo" label="协调阶段号" />
                    <el-table-column label="记录时间">
                      <template #default="scope">
                        {{ new Date(scope.row?.recordTime).toLocaleString() }}
                      </template>
                    </el-table-column>
                    <el-table-column label="阶段配时信息">
                      <template #default="scope">
                        <el-table :data="scope.row?.stageTimingList?.stageTiming" border style="width: 100%">
                          <el-table-column prop="stageNo" label="阶段号" width="80" />
                          <el-table-column prop="green" label="绿灯" width="80">
                            <template #default="stage">
                              {{ stage.row?.green }}秒
                            </template>
                          </el-table-column>
                          <el-table-column prop="yellow" label="黄灯" width="80">
                            <template #default="stage">
                              {{ stage.row?.yellow }}秒
                            </template>
                          </el-table-column>
                          <el-table-column prop="allRed" label="全红" width="80">
                            <template #default="stage">
                              {{ stage.row?.allRed }}秒
                            </template>
                          </el-table-column>
                          <el-table-column prop="minGreen" label="最小绿" width="80">
                            <template #default="stage">
                              {{ stage.row?.minGreen }}秒
                            </template>
                          </el-table-column>
                          <el-table-column prop="maxGreen" label="最大绿" width="80">
                            <template #default="stage">
                              {{ stage.row?.maxGreen }}秒
                            </template>
                          </el-table-column>
                        </el-table>
                      </template>
                    </el-table-column>
                    <el-table-column label="原始信息">
                      <template #default="scope">
                        {{ JSON.stringify(scope.row) }}
                      </template>
                    </el-table-column>
                  </el-table>
                </template>
                <template v-else>
                  <el-empty description="暂无方案信息" />
                </template>

                <h3>上周期信息</h3>
                <el-table :data="[runInfo.cycleInfo]" border style="width: 100%">
                  <el-table-column label="周期长度">
                    <template #default="scope">
                      {{ scope.row?.lastCycleLen }}秒
                    </template>
                  </el-table-column>
                  <el-table-column label="记录时间">
                    <template #default="scope">
                      {{ new Date(scope.row?.recordTime).toLocaleString() }}
                    </template>
                  </el-table-column>
                  <el-table-column prop="startTime" label="开始时间" />
                  <el-table-column label="原始信息">
                    <template #default="scope">
                      {{ JSON.stringify(scope.row) }}
                    </template>
                  </el-table-column>
                </el-table>

                <h3>当前阶段</h3>
                <el-table :data="[runInfo.stageInfo]" border style="width: 100%">
                  <el-table-column prop="curStageNo" label="当前阶段号" />
                  <el-table-column label="当前阶段时长">
                    <template #default="scope">
                      {{ scope.row?.curStageLen }}秒
                    </template>
                  </el-table-column>
                  <el-table-column prop="curStageStartTime" label="当前阶段开始时间" />
                  <el-table-column prop="lastStageNo" label="上一阶段号" />
                  <el-table-column label="上一阶段时长">
                    <template #default="scope">
                      {{ scope.row?.lastStageLen }}秒
                    </template>
                  </el-table-column>
                  <el-table-column label="记录时间">
                    <template #default="scope">
                      {{ new Date(scope.row?.recordTime).toLocaleString() }}
                    </template>
                  </el-table-column>
                  <el-table-column label="原始信息">
                    <template #default="scope">
                      {{ JSON.stringify(scope.row) }}
                    </template>
                  </el-table-column>
                </el-table>
              </div>
              <h3>灯色信息  {{ new Date(runInfo.crossSignalGroupStatus?.recordTime).toLocaleString() }}</h3>

              <pre class="json-display" style="white-space: pre-wrap; word-wrap: break-word;">{{ JSON.stringify(runInfo.crossSignalGroupStatus) }}</pre>
             
             
              <el-table :data="runInfo.crossSignalGroupStatus?.signalGroupStatusList?.signalGroupStatus" border style="width: 100%" size="small">
                <el-table-column prop="signalGroupNo" label="信号组号" />
                <el-table-column label="灯色状态">
                  <template #default="scope">
                    {{ formatLampStatus(scope.row.lampStatus )}}
                  </template>
                </el-table-column>
                <el-table-column label="剩余时间">
                  <template #default="scope">
                    {{ scope.row.remainTime }}秒
                  </template>
                </el-table-column>
             
              </el-table>
           
           
              <!-- 原有九宫格内容 -->
              <div class="cross-map-container">
                <div class="cross-map-grid">
                  <!-- 九宫格 Cells -->
                  <div class="grid-cell-numbered grid-cell-1">1</div>
                  <div class="grid-cell-numbered grid-cell-2">
                    <div class="cell-number">2</div>
                    <div class="cell-content-wrapper">
                      <!-- 北进口区域 -->
                      <div class="lane-section">
                                                  <div class="direction-container north">
                          <div class="lane-container">
                            <template v-for="lane in getLanesByDirectionAndAttribute(1, 0)" :key="lane.laneNo">
                              <div class="lane-div import-lane" :title="getLaneTooltip(lane)">
                                <span>{{ formatMovement(lane.movement) }}</span>
                                <span>{{ lane.laneNo }}</span>
                                <span>{{ lane.azimuth }}°</span>
                              </div>
                            </template>
                          </div>
                        </div>
                      </div>
                       <!-- 南出口（在北面）区域 -->
                      <div class="lane-section">
                        <div class="direction-container north">
                          <div class="lane-container">
                            <template v-for="lane in getLanesByDirectionAndAttribute(5, 1)" :key="lane.laneNo">
                              <div class="lane-div export-lane" :title="getLaneTooltip(lane)">
                                <span>{{ lane.laneNo }}</span>
                                <span>{{ formatMovement(lane.movement) }}</span>
                                <span>{{ lane.azimuth }}°</span>
                              </div>
                            </template>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="grid-cell-numbered grid-cell-3">3</div>
                  <div class="grid-cell-numbered grid-cell-4">
                    <div class="cell-number">4</div>
                    <div class="cell-content-wrapper column-layout">
                        <!-- 东出口（在西面）区域 -->
                        <div class="lane-section">
                        <div class="direction-container west"> <!-- 东出口车辆向西驶出 -->
                          <div class="lane-container">
                            <template v-for="lane in getLanesByDirectionAndAttribute(3, 1)" :key="lane.laneNo">
                              <div class="lane-div export-lane" :title="getLaneTooltip(lane)">
                                <span>{{ lane.laneNo }}</span>
                                <span>{{ formatMovement(lane.movement) }}</span>
                                <span>{{ lane.azimuth }}°</span>
                              </div>
                            </template>
                          </div>
                        </div>
                      </div>
                     <!-- 西进口区域 -->
                      <div class="lane-section">
                        <div class="direction-container west">
                          <div class="lane-container">
                            <template v-for="lane in getLanesByDirectionAndAttribute(7, 0)" :key="lane.laneNo">
                              <div class="lane-div import-lane" :title="getLaneTooltip(lane)" style="flex-direction: row; gap: 4px;">
                                <span>{{ formatMovement(lane.movement) }}</span>
                                <span>{{ lane.laneNo }}</span>
                                <span>{{ lane.azimuth }}°</span>
                              </div>
                            </template>
                          </div>
                        </div>
                      </div>
                     
                    </div>
                  </div>
                  <div class="grid-cell-numbered grid-cell-5 cross-center"><div class="cell-number">5</div>
                    <!-- 中心区域 -->
                    <div class="center-sub-cells">
                      <div class="center-sub-cell">
                        <div class="cell-number">北进口</div>
                        <div class="lamp-group-container">
                          <template v-for="lampGroup in getLampGroupsByDirection(1)" :key="lampGroup.lampGroupNo">
                              <div class="lamp-group-div" :title="getLampGroupTooltip(lampGroup)" :class="{
                                'status-red': lampGroup.status?.[0] === '2',
                                'status-yellow': lampGroup.status?.[1] === '2',
                                'status-green': lampGroup.status?.[2] === '2',
                                'status-flash': lampGroup.status?.includes('3')
                              }">
                                <span class="lamp-number">{{ lampGroup.lampGroupNo }}</span>
                                <span class="lamp-shape">
                                  <template v-if="lampGroup.type == '10'">
                                    <div class="dot-shape"></div>
                                  </template>
                                  <template v-else-if="lampGroup.type == '21'">
                                    ↓
                                  </template>
                                  <template v-else-if="lampGroup.type == '22'">
                                    →
                                  </template>
                                  <template v-else-if="lampGroup.type == '23'">
                                     ←
                                  </template>
                                  <template v-else-if="lampGroup.type == '80'">
                                    ↻
                                  </template>
                                </span>
                              </div>
                            </template>
                        </div>
                      </div>
                    
                      <div class="center-sub-cell"> <div class="cell-number">东进口</div><div class="lamp-group-container" style="flex-direction: column;">
                          <template v-for="lampGroup in getLampGroupsByDirection(3)" :key="lampGroup.lampGroupNo">
                              <div class="lamp-group-div" :title="getLampGroupTooltip(lampGroup)" :class="{
                                'status-red': lampGroup.status?.[0] === '2',
                                'status-yellow': lampGroup.status?.[1] === '2',
                                'status-green': lampGroup.status?.[2] === '2',
                                'status-flash': lampGroup.status?.includes('3')
                              }">
                                <span class="lamp-number">{{ lampGroup.lampGroupNo }}</span>
                                <span class="lamp-shape">
                                  <template v-if="lampGroup.type == '10'">
                                    <div class="dot-shape"></div>
                                  </template>
                                  <template v-else-if="lampGroup.type == '21'">
                                    ← 
                                  </template>
                                  <template v-else-if="lampGroup.type == '22'">
                                    ↓ 
                                  </template>
                                  <template v-else-if="lampGroup.type == '23'">
                                     ↑
                                  </template>
                                  <template v-else-if="lampGroup.type == '80'">
                                    ↻
                                  </template>
                                </span>
                              </div>
                            </template>
                        </div></div>
                      <div class="center-sub-cell center-sub-cell-3">
                        <div class="cell-number">西进口</div><div class="lamp-group-container" style="flex-direction: column;">
                          <template v-for="lampGroup in getLampGroupsByDirection(7)" :key="lampGroup.lampGroupNo">
                              <div class="lamp-group-div" :title="getLampGroupTooltip(lampGroup)" :class="{
                                'status-red': lampGroup.status?.[0] === '2',
                                'status-yellow': lampGroup.status?.[1] === '2',
                                'status-green': lampGroup.status?.[2] === '2',
                                'status-flash': lampGroup.status?.includes('3')
                              }">
                                <span class="lamp-number">{{ lampGroup.lampGroupNo }}</span>
                                <span class="lamp-shape">
                                  <template v-if="lampGroup.type == '10'">
                                    <div class="dot-shape"></div>
                                  </template>
                                  <template v-else-if="lampGroup.type == '21'">
                                    →
                                  </template>
                                  <template v-else-if="lampGroup.type == '22'">
                                    ↑
                                  </template>
                                  <template v-else-if="lampGroup.type == '23'">
                                     ↓ 
                                  </template>
                                  <template v-else-if="lampGroup.type == '80'">
                                    ↻
                                  </template>
                                </span>
                              </div>
                            </template>
                        </div>
                      </div>
                      <div class="center-sub-cell"> 
                        <div class="cell-number">南进口</div>
                        <div class="lamp-group-container">
                          <template v-for="lampGroup in getLampGroupsByDirection(5)" :key="lampGroup.lampGroupNo">
                              <div class="lamp-group-div" :title="getLampGroupTooltip(lampGroup)" :class="{
                                'status-red': lampGroup.status?.[0] === '2',
                                'status-yellow': lampGroup.status?.[1] === '2',
                                'status-green': lampGroup.status?.[2] === '2',
                                'status-flash': lampGroup.status?.includes('3')
                              }">
                                <span class="lamp-number">{{ lampGroup.lampGroupNo }}</span>
                                <span class="lamp-shape">
                                  <template v-if="lampGroup.type == '10'">
                                    <div class="dot-shape"></div>
                                  </template>
                                  <template v-else-if="lampGroup.type == '21'">
                                    ↑
                                  </template>
                                  <template v-else-if="lampGroup.type == '22'">
                                    ←
                                  </template>
                                  <template v-else-if="lampGroup.type == '23'">
                                    →
                                  </template>
                                  <template v-else-if="lampGroup.type == '80'">
                                    ↻
                                  </template>
                                </span>
                              </div>
                            </template>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="grid-cell-numbered grid-cell-6">
                    <div class="cell-number">6</div>
                    <div class="cell-content-wrapper column-layout">
                    <!-- 东进口区域 -->
                      <div class="lane-section">
                        <div class="direction-container east">
                          <div class="lane-container">
                            <template v-for="lane in getLanesByDirectionAndAttribute(3, 0)" :key="lane.laneNo">
                              <div class="lane-div import-lane" :title="getLaneTooltip(lane)" style="flex-direction: row; gap: 4px;">
                                <span>{{ lane.laneNo }}</span>
                                <span>{{ formatMovement(lane.movement) }}</span>
                                <span>{{ lane.azimuth }}°</span>
                              </div>
                            </template>
                          </div>
                        </div>
                      </div>
                     <!-- 西出口（在东面）区域 -->
                      <div class="lane-section">
                        <div class="direction-container east"> <!-- 西出口车辆向东驶出 -->
                          <div class="lane-container">
                            <template v-for="lane in getLanesByDirectionAndAttribute(7, 1)" :key="lane.laneNo">
                              <div class="lane-div export-lane" :title="getLaneTooltip(lane)" >
                                <span>{{ lane.laneNo }}</span>
                                <span>{{ formatMovement(lane.movement) }}</span>
                                <span>{{ lane.azimuth }}°</span>
                              </div>
                            </template>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="grid-cell-numbered grid-cell-7">7</div>
                  <div class="grid-cell-numbered grid-cell-8">
                    <div class="cell-number">8</div>
                    <div class="cell-content-wrapper">
                     <!-- 北出口（在南面）区域 -->
                      <div class="lane-section">
                        <div class="direction-container south">
                          <div class="lane-container">
                            <template v-for="lane in getLanesByDirectionAndAttribute(1, 1)" :key="lane.laneNo">
                              <div class="lane-div export-lane" :title="getLaneTooltip(lane)">
                                <span>{{ lane.laneNo }}</span>
                                <span>{{ formatMovement(lane.movement) }}</span>
                                <span>{{ lane.azimuth }}°</span>
                              </div>
                            </template>
                          </div>
                        </div>
                      </div>
                      <!-- 南进口区域 -->
                       <div class="lane-section">
                        <div class="direction-container south">
                          <div class="lane-container">
                            <template v-for="lane in getLanesByDirectionAndAttribute(5, 0)" :key="lane.laneNo">
                              <div class="lane-div import-lane" :title="getLaneTooltip(lane)">
                                <span>{{ lane.laneNo }}</span>
                                <span>{{ formatMovement(lane.movement) }}</span>
                                <span>{{ lane.azimuth }}°</span>
                              </div>
                            </template>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="grid-cell-numbered grid-cell-9">9</div>
                </div>
                <!-- 添加图例 -->
                <div class="map-legend">
                  <h4>图例说明</h4>
                  <div class="legend-item">
                    <div class="legend-color" style="background-color: #409EFF;"></div>
                    <span>进口车道</span>
                  </div>
                  <div class="legend-item">
                    <div class="legend-color" style="background-color: #67C23A;"></div>
                    <span>出口车道</span>
                  </div>
                  <div class="legend-item">
                    <div class="legend-color" style="background-color: #67C23A;"></div>
                    <span>信号灯组(绿色)</span>
                  </div>
                  <phase v-if="runInfo.phaseInfo" :phaseImage="runInfo.phaseInfo"></phase>
                </div>
              </div>
            </el-tab-pane>
          </el-tabs>
        </div>
      </template>
      <div v-else class="empty-state">
        <el-empty description="请选择一个键" />
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 基础布局样式 */
.browser-container {
  height: 100vh;
  display: flex;
  background-color: #f5f7fa;
}

.key-list {
  width: 300px;
  border-right: 1px solid #dcdfe6;
  background-color: #fff;
  display: flex;
  flex-direction: column;
}

.search-box {
  padding: 10px;
  border-bottom: 1px solid #dcdfe6;
}

.data-view {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

.key-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.key-header h3 {
  margin: 0;
  color: #303133;
}

.value-display {
  background-color: #fff;
  border-radius: 4px;
  padding: 20px;
}

.value-content {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-all;
  font-family: monospace;
  background-color: #f8f9fa;
  padding: 10px;
  border-radius: 4px;
}

.empty-state {
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}

.custom-tree-node {
  display: flex;
  align-items: center;
  gap: 4px;
}

.el-collapse {
  margin-top: 10px;
}

.el-collapse-item :deep(.el-collapse-item__header) {
  font-weight: bold;
}

.el-collapse-item :deep(.el-collapse-item__content) {
  padding: 10px;
}

.el-descriptions {
  margin: 10px 0;
}

.el-descriptions-item__label {
  font-weight: bold;
  width: 120px;
}

.el-descriptions-item__content {
  word-break: break-all;
}

.detail-grid {
  display: grid;
  gap: 20px;
  padding: 20px 0;
  max-width: 100%;
}

.detail-card {
  height: fit-content;
  min-width: 0; /* 防止卡片内容溢出 */
}

.detail-card :deep(.el-card__body) {
  padding: 10px;
  overflow-x: auto; /* 如果内容过宽，允许横向滚动 */
}

.el-descriptions {
  margin: 0;
}

.el-descriptions-item__label {
  font-weight: bold;
  width: 120px;
}

.el-descriptions-item__content {
  word-break: break-all;
}

.value-with-type {
  gap: 8px;
}

.type-tag {
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  font-family: monospace;
  white-space: nowrap;
  flex-shrink: 0;
}

.type-string {
  background-color: #e6f7ff;
  color: #1890ff;
  border: 1px solid #91d5ff;
}

.type-number {
  background-color: #f6ffed;
  color: #52c41a;
  border: 1px solid #b7eb8f;
}

.type-boolean {
  background-color: #fff7e6;
  color: #fa8c16;
  border: 1px solid #ffd591;
}

.type-object {
  background-color: #f9f0ff;
  color: #722ed1;
  border: 1px solid #d3adf7;
}

.type-null {
  background-color: #f5f5f5;
  color: #8c8c8c;
  border: 1px solid #d9d9d9;
}

.type-undefined {
  background-color: #fff1f0;
  color: #f5222d;
  border: 1px solid #ffa39e;
}

.value-content {
  flex: 1;
  word-break: break-all;
  white-space: pre-wrap;
  font-family: monospace;
}

/* 确保 v-html 内容中的样式生效 */
:deep(.value-with-type) {
  gap: 8px;
}

:deep(.type-tag) {
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  font-family: monospace;
  white-space: nowrap;
  flex-shrink: 0;
}

:deep(.type-string) {
  background-color: #e6f7ff;
  color: #1890ff;
  border: 1px solid #91d5ff;
}

:deep(.type-number) {
  background-color: #f6ffed;
  color: #52c41a;
  border: 1px solid #b7eb8f;
}

:deep(.type-boolean) {
  background-color: #fff7e6;
  color: #fa8c16;
  border: 1px solid #ffd591;
}

:deep(.type-object) {
  background-color: #f9f0ff;
  color: #722ed1;
  border: 1px solid #d3adf7;
}

:deep(.type-null) {
  background-color: #f5f5f5;
  color: #8c8c8c;
  border: 1px solid #d9d9d9;
}

:deep(.type-undefined) {
  background-color: #fff1f0;
  color: #f5222d;
  border: 1px solid #ffa39e;
}

:deep(.value-content) {
  flex: 1;
  word-break: break-all;
  white-space: pre-wrap;
  font-family: monospace;
}

/* 添加标签页数量样式 */
:deep(.el-tabs__item) {
  display: flex;
  align-items: center;
  gap: 4px;
}

:deep(.el-tabs__item .count) {
  background-color: #f0f0f0;
  padding: 2px 6px;
  border-radius: 10px;
  font-size: 12px;
  color: #666;
}

.field-description {
  font-size: 12px;
  color: #666;
  background-color: #f8f9fa;
  padding: 8px;
  border-radius: 4px;
  margin-bottom: 8px;
  line-height: 1.5;
}

:deep(.el-descriptions__label) {
  font-weight: bold;
  min-width: 160px;  /* 减小标签宽度 */
  max-width: 160px;  /* 限制最大宽度 */
  padding-right: 12px;  /* 增加标签和内容之间的间距 */
}

:deep(.el-descriptions__content) {
  padding: 12px;
  min-width: 300px;  /* 设置内容最小宽度 */
  word-break: break-all;  /* 允许在任意字符间换行 */
}

/* 视图切换样式 */
.view-switch {
  margin-bottom: 16px;
  display: flex;
  justify-content: flex-end;
}

/* JSON视图样式 */
.json-view {
  background-color: #f5f7fa;
  border-radius: 4px;
  padding: 16px;
  margin-top: 16px;
}

.json-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.json-title {
  font-size: 14px;
  font-weight: 500;
  color: #606266;
}

.json-content {
  background-color: #fff;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  padding: 16px;
  margin: 0;
  font-family: 'Courier New', Courier, monospace;
  font-size: 13px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-wrap: break-word;
  max-height: 600px;
  overflow-y: auto;
}

/* 确保表格视图和JSON视图的间距一致 */
.detail-grid {
  margin-top: 16px;
}

/* 添加校验相关样式 */
:deep(.invalid-value) {
  background-color: #fff2f0;
  border: 1px solid #ffccc7;
  border-radius: 4px;
  padding: 4px;
}

:deep(.validation-message) {
  color: #ff4d4f;
  font-size: 12px;
  margin-top: 4px;
  padding: 4px;
  background-color: #fff1f0;
  border-radius: 2px;
}

/* 确保校验消息在JSON视图中也正确显示 */
.json-content :deep(.invalid-value) {
  background-color: #fff2f0;
  border: 1px solid #ffccc7;
  border-radius: 4px;
  padding: 4px;
}

.json-content :deep(.validation-message) {
  color: #ff4d4f;
  font-size: 12px;
  margin-top: 4px;
  padding: 4px;
  background-color: #fff1f0;
  border-radius: 2px;
}

/* 确保校验消息在灯组信息中也正确显示 */
.detail-card :deep(.invalid-value) {
  background-color: #fff2f0;
  border: 1px solid #ffccc7;
  border-radius: 4px;
  padding: 4px;
}

.detail-card :deep(.validation-message) {
  color: #ff4d4f;
  font-size: 12px;
  margin-top: 4px;
  padding: 4px;
  background-color: #fff1f0;
  border-radius: 2px;
  white-space: pre-line;
}

.signal-group-status {
  margin: 8px 0;
  padding: 8px;
  background-color: #f8f9fa;
  border-radius: 4px;
}

.signal-group-status strong {
  color: #409EFF;
  margin-right: 8px;
}

.signal-group-status :deep(.value-content) {
  white-space: pre-line;
  margin-top: 4px;
}

/* 路口图样式 */
.cross-map-container {
  display: flex;
  gap: 20px;
  padding: 20px;
  background-color: #fff;
  border-radius: 4px;
  justify-content: center;
}

.cross-map-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: repeat(3, 1fr);
  width: 800px;
  height: 800px;
  border: 2px solid #dcdfe6;
  background-color: #f0f0f0;
  position: relative;
}

.grid-cell-numbered {
  border: 1px solid #eee;
  display: flex;
  flex-direction: column; /* 保持列方向以便放置编号和内容 */
  justify-content: center;
  align-items: center;
  overflow: hidden;
  padding: 5px;
  position: relative;
  font-size: 20px;
  font-weight: bold;
  color: #606266;
}

.cell-number {
  position: absolute;
  top: 5px;
  left: 5px;
  font-size: 14px;
  font-weight: bold;
  color: #303133;
  z-index: 1; /* 确保编号在内容上方 */
}

.cell-content-wrapper {
  display: flex;
  width: 100%;
  height: 100%;
  gap: 5px; /* 内容区域之间的间距 */
  flex-direction: row; /* Default to row layout */
}

.cell-content-wrapper.column-layout {
  flex-direction: column; /* Vertical layout */
}

.lane-section {
  flex: 1; /* 平均分配空间 */
  display: flex;
  justify-content: center;
  align-items: center;
}

/* 方向容器样式 */
.direction-container {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
}

/* 车道容器样式 */
.lane-container {
  display: flex;
  gap: 4px; /* 减小车道之间的间距 */
  padding: 0; /* 移除内边距 */
  flex-wrap: nowrap;  /* 防止换行 */
  justify-content: center;
  align-items: center;
  min-height: 0; /* 允许根据内容收缩 */
  min-width: 0;
  width: 100%;  /* 确保容器占满整个方向区域 */
  height: 100%;
}

/* 不同方向的车道布局 */
.direction-container.north .lane-container,
.direction-container.south .lane-container {
  flex-direction: row;  /* 北向进口/南向出口：从左到右 */
}

.direction-container.east .lane-container,
.direction-container.west .lane-container {
  flex-direction: column;  /* 东向进口/西向出口：从上到下 */
}

/* 中心区域样式 */
.cross-center {
  background-color: #cccccc;
  border: 2px solid #999;
  display: flex; /* 中心区域也使用flex */
  flex-direction: column; /* 保持列方向以便放置编号 */
  justify-content: center;
  align-items: center;
}

.center-sub-cells {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  grid-template-rows: repeat(2, 1fr);
  width: 100%;
  height: 100%;
}

.center-sub-cell {
  border: 1px dashed #999; /* 使用虚线区分 */
  display: flex;
  flex-direction: column; /* 内部元素垂直排列 */
  justify-content: center;
  align-items: center;
  font-size: 16px;
  font-weight: normal;
  color: #303133;
  position: relative; /* 为内部编号定位 */
}

.center-sub-cell .cell-number {
  position: absolute;
  top: 5px;
  left: 5px;
  font-size: 12px; /* 子格编号小一些 */
  font-weight: bold;
  color: #303133;
  z-index: 1;
}

.lamp-group-container {
  display: flex;
  flex-wrap: wrap; /* 允许换行 */
  gap: 4px;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  padding-top: 15px; /* 为编号留出空间 */
  overflow: auto; /* 如果内容过多，允许滚动 */
}

.lamp-group-div {
  background-color: #67C23A; /* 绿色背景 */
  color: white;
  padding: 0px 0px;
  border-radius: 1px;
  font-size: 12px;
  white-space: nowrap;
  cursor: help;
  box-shadow: 0 1px 2px rgba(0,0,0,0.1);
  transition: all 0.3s ease;
  display: flex; /* 使用flexbox */
  flex-direction: column; /* 元素垂直排列 */
  align-items: center;
  justify-content: center;
  min-width: 25px; /* 给灯组div一个最小尺寸 */
  min-height: 25px;
}

.lamp-group-div:hover {
   transform: scale(1.05);
  box-shadow: 0 2px 4px rgba(0,0,0,0.2);
}

.lane-div {
  color: white;
  padding: 4px 8px; /* 调整内边距 */
  border-radius: 4px;
  font-size: 12px; /* 调整字体大小 */
  white-space: nowrap;
  /* 移除最小尺寸限制，允许flex box控制大小 */
  /* min-width: 30px; */
  /* min-height: 30px; */
  text-align: center;
  cursor: help;
  box-shadow: 0 1px 2px rgba(0,0,0,0.1); /* 调整阴影 */
  transition: all 0.3s ease;
  margin: 1px; /* 调整外边距 */
  display: flex;  /* 使用flex布局 */
  flex-direction: column; /* 垂直排列 */
  align-items: center;
  justify-content: center;
  /* 根据父容器（lane-container）的flex方向调整尺寸 */
  flex-grow: 1; /* 允许在主轴方向上增长 */
}

/* 进口车道样式 */
.lane-div.import-lane {
  background-color: #409EFF; /* 蓝色表示进口 */
}

/* 出口车道样式 */
.lane-div.export-lane {
  background-color: #67C23A; /* 绿色表示出口 */
}

/* 当父容器是行方向时，填充高度 */
.direction-container.north .lane-container .lane-div,
.direction-container.south .lane-container .lane-div {
    height: 100%;
    width: auto; /* 宽度根据内容和flex item自动调整 */
}

/* 当父容器是列方向时，填充宽度 */
.direction-container.east .lane-container .lane-div,
.direction-container.west .lane-container .lane-div {
    width: 100%;
    height: auto; /* 高度根据内容和flex item自动调整 */
}

.lane-div:hover {
  transform: scale(1.05);
  box-shadow: 0 2px 4px rgba(0,0,0,0.2);
}

/* 图例样式 */
.map-legend {
  width: 200px;
  padding: 16px;
  background-color: #f8f9fa;
  border-radius: 4px;
  border: 1px solid #dcdfe6;
  align-self: flex-start;
  margin-top: 20px;
}

.map-legend h4 {
  margin: 0 0 12px 0;
  color: #303133;
}

.legend-item {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
}

.legend-color {
  width: 20px;
  height: 20px;
  margin-right: 8px;
  border-radius: 2px;
}

.legend-item span {
  font-size: 14px;
  color: #606266;
}

.filter-bar {
  margin-bottom: 16px;
  display: flex;
  align-items: center;
  gap: 16px;
  flex-wrap: wrap;
}

.filter-bar .el-select {
  width: 200px;
}

.filter-bar .column-select {
  margin-left: auto;
}

.filter-count {
  color: #909399;
  font-size: 14px;
}

/* 当没有数据时显示提示 */
.no-data {
  text-align: center;
  padding: 20px;
  color: #909399;
  font-size: 14px;
}

/* 添加不同列数的样式类 */
.detail-grid.columns-2 {
  grid-template-columns: repeat(2, 1fr);
}

.detail-grid.columns-3 {
  grid-template-columns: repeat(3, 1fr);
}

.detail-grid.columns-4 {
  grid-template-columns: repeat(4, 1fr);
}

.detail-grid.columns-6 {
  grid-template-columns: repeat(6, 1fr);
}

/* 添加灯组编号和形状的样式 */
.lamp-number {
  font-size: 10px;
  font-weight: bold;
}

.lamp-shape {
  font-size: 14px; /* 箭头符号稍大 */
  line-height: 1; /* 调整行高 */
}

/* 圆点形状 */
.dot-shape {
  width: 8px;
  height: 8px;
  background-color: white; /* 圆点颜色 */
  border-radius: 50%; /* 使其成为圆形 */
}
/* 灯组状态样式 */
.status-red {
  background-color: #d32f2f; /* 更深的红色 */
  border: 2px solid #b71c1c; /* 添加深色边框增加对比度 */
}

.status-yellow {
  background-color: #ffa000; /* 更亮的黄色 */
  border: 2px solid #f57f17; /* 添加深色边框增加对比度 */
}

.status-green {
  background-color: #2e7d32; /* 更深的绿色 */
  border: 2px solid #1b5e20; /* 添加深色边框增加对比度 */
}

.status-flash {
  animation: flash 1s infinite;
}

@keyframes flash {
  0% { opacity: 1; }
  50% { opacity: 0.3; }
  100% { opacity: 1; }
}

</style>

<script setup>
import { ref, onMounted, computed, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search, Document, Refresh, Delete } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/tauri'
import { getPhaseInfo } from '../utils/cross-util'
import phase from '../components/phase.vue'
import { useRouter } from 'vue-router'

const router=useRouter()






const searchPattern = ref('')
const keyTree = ref([])
const selectedKey = ref('')
const keyType = ref('')
const ttl = ref(0)
const value = ref(null)

const defaultProps = {
  children: 'children',
  label: 'name'
}

const activeTab = ref('basic')

const lampGroupDetails = ref({})
const laneDetails = ref({})
const pedestrianDetails = ref({})
const stageDetails = ref({})
const detDetails = ref({})
const planDetails = ref({})
const signalGroupDetails = ref({})
const scheduleDetails = ref({})
const dayPlanDetails = ref({})


const lampGroupNos = computed(() => {
  if (!value.value?.lampGroupNoList?.lampGroupNo) return []
  return value.value.lampGroupNoList.lampGroupNo
})

const laneNos = computed(() => {
  if (!value.value?.laneNoList?.laneNo) return []
  return value.value.laneNoList.laneNo
})

const pedestrianNos = computed(() => {
  if (!value.value?.pedestrianNoList?.pedestrianNo) return []
  return value.value.pedestrianNoList.pedestrianNo
})

const stageNos = computed(() => {
  if (!value.value?.stageNoList?.stageNo) return []
  return value.value.stageNoList.stageNo
})

const detNos = computed(() => {
  if (!value.value?.detNoList?.detNo) return []
  return value.value.detNoList.detNo
})

const planNos = computed(() => {
  if (!value.value?.planNoList?.planNo) return []
  return value.value.planNoList.planNo
})

const signalGroupNos = computed(() => {
  if (!value.value?.signalGroupNoList?.signalGroupNo) return []
  return value.value.signalGroupNoList.signalGroupNo
})

const scheduleNos = computed(() => {
  if (!value.value?.scheduleNoList?.scheduleNo) return []
  return value.value.scheduleNoList.scheduleNo
})
const dayPlanNos = computed(() => {
  if (!value.value?.dayPlanNoList?.dayPlanNo) return []
  return value.value.dayPlanNoList.dayPlanNo
})


// 添加视图模式状态
const basicViewMode = ref('table')

// 添加校验规则
const validationRules = {
  crossID: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口编号不能为空' }
      // 检查是否为12位数字
      const pattern = /^\d{12}$/
      if (!pattern.test(value)) {
        return { valid: false, message: '路口编号格式错误，应为12位数字' }
      }
      return { valid: true }
    }
  },
  crossName: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口名称不能为空' }
      if (value.length > 100) {
        return { valid: false, message: '路口名称长度不能超过100' }
      }
      return { valid: true }
    }
  },
  feature: {
    validate: (value) => {
      if (value === undefined || value === null) return { valid: false, message: '路口形状不能为空' }
      const validValues = [10, 12, 23, 24, 35, 36, 39, 40, 50, 51, 52, 61, 90]
      if (!validValues.includes(Number(value))) {
        return { valid: false, message: '路口形状值无效，应为10、12、23、24、35、36、39、40、50、51、52、61或90' }
      }
      return { valid: true }
    }
  },
  grade: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口等级不能为空' }
      const validValues = ['11', '12', '13', '21', '22', '31', '99']
      if (!validValues.includes(value)) {
        return { valid: false, message: '路口等级值无效' }
      }
      return { valid: true }
    }
  },
  detNoList: {
    validate: (value) => {
      // 可以为空
      if (!value) return { valid: true }
      if (!Array.isArray(value?.detNo)) {
        return { valid: false, message: '检测器序号列表格式错误' }
      }
      return { valid: true }
    }
  },
  laneNoList: {
    validate: (value) => {
      if (!value?.laneNo?.length) {
        return { valid: false, message: '车道序号列表不能为空，至少包含1个车道序号' }
      }
      return { valid: true }
    }
  },
  pedestrianNoList: {
    validate: (value) => {
      // 可以为空
      if (!value) return { valid: true }
      if (!Array.isArray(value?.pedestrianNo)) {
        return { valid: false, message: '人行横道序号列表格式错误' }
      }
      return { valid: true }
    }
  },
  lampGroupNoList: {
    validate: (value) => {
      if (!value?.lampGroupNo?.length) {
        return { valid: false, message: '信号灯组序号列表不能为空，至少包含1个灯组序号' }
      }
      return { valid: true }
    }
  },
  signalGroupNoList: {
    validate: (value) => {
      if (!value?.signalGroupNo?.length) {
        return { valid: false, message: '信号组序号列表不能为空，至少包含1个信号组序号' }
      }
      return { valid: true }
    }
  },
  greenConflictMatrix: {
    validate: (value) => {
      if (!value) return { valid: false, message: '绿冲突矩阵不能为空' }
      // 检查是否只包含0和1
      if (!/^[01]+$/.test(value)) {
        return { valid: false, message: '绿冲突矩阵只能包含0和1' }
      }
      // 检查长度是否为信号组数量的平方
      const signalGroupCount = value.value?.signalGroupNoList?.signalGroupNo?.length || 0
      if (value.length !== signalGroupCount * signalGroupCount) {
        return { valid: false, message: '绿冲突矩阵长度必须等于信号组数量的平方' }
      }
      return { valid: true }
    }
  },
  stageNoList: {
    validate: (value) => {
      if (!value?.stageNo?.length) {
        return { valid: false, message: '阶段号列表不能为空，至少包含1个阶段号' }
      }
      return { valid: true }
    }
  },
  planNoList: {
    validate: (value) => {
      if (!value?.planNo?.length) {
        return { valid: false, message: '配时方案序号列表不能为空，至少包含1个配时方案序号' }
      }
      return { valid: true }
    }
  },
  dayPlanNoList: {
    validate: (value) => {
      if (!value?.dayPlanNo?.length) {
        return { valid: false, message: '日计划号列表不能为空，至少包含1个日计划号' }
      }
      return { valid: true }
    }
  },
  scheduleNoList: {
    validate: (value) => {
      if (!value?.scheduleNo?.length) {
        return { valid: false, message: '调度号列表不能为空，至少包含1个调度号' }
      }
      return { valid: true }
    }
  },
  longitude: {
    validate: (value) => {
      if (value === undefined || value === null) return { valid: false, message: '经度不能为空' }
      const num = Number(value)
      if (isNaN(num) || num < -180 || num > 180) {
        return { valid: false, message: '经度值无效，应在-180到180之间' }
      }
      return { valid: true }
    }
  },
  latitude: {
    validate: (value) => {
      if (value === undefined || value === null) return { valid: false, message: '纬度不能为空' }
      const num = Number(value)
      if (isNaN(num) || num < -90 || num > 90) {
        return { valid: false, message: '纬度值无效，应在-90到90之间' }
      }
      return { valid: true }
    }
  },
  altitude: {
    validate: (value) => {
      // 可以为空
      if (value === undefined || value === null) return { valid: true }
      const num = Number(value)
      if (isNaN(num)) {
        return { valid: false, message: '海拔高度必须是数字' }
      }
      return { valid: true }
    }
  }
}

// 添加灯组参数的校验规则
const lampGroupValidationRules = {
  crossID: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口编号不能为空' }
      const pattern = /^\d{12}$/
      if (!pattern.test(value)) {
        return { valid: false, message: '路口编号格式错误，应为12位数字' }
      }
      return { valid: true }
    }
  },
  lampGroupNo: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '信号灯组序号不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 1 || num > 99) {
        return { valid: false, message: '信号灯组序号必须在1-99范围内' }
      }
      return { valid: true }
    }
  },
  direction: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '进口方向不能为空' }
      }
      const validValues = [1, 2, 3, 4, 5, 6, 7, 8, 9]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '进口方向值无效，应为：\n1：北\n2：东北\n3：东\n4：东南\n5：南\n6：西南\n7：西\n8：西北\n9：其他'
        }
      }
      return { valid: true }
    }
  },
  type: {
    validate: (value) => {
      if (!value) return { valid: false, message: '信号灯组类型不能为空' }
      // 这里需要根据GB/T 39900—2021的A.18.6添加具体的类型值校验
      // 暂时只做非空校验
      return { valid: true }
    }
  }
}

// 添加检测器参数的校验规则
const detectorValidationRules = {
  crossID: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口编号不能为空' }
      const pattern = /^\d{12}$/
      if (!pattern.test(value)) {
        return { valid: false, message: '路口编号格式错误，应为12位数字' }
      }
      return { valid: true }
    }
  },
  detectorNo: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '检测器序号不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 1 || num > 999) {
        return { valid: false, message: '检测器序号必须在1-999范围内' }
      }
      return { valid: true }
    }
  },
  type: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '检测器类型不能为空' }
      }
      const validValues = [1, 2, 3, 4, 5, 6, 9]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '检测器类型值无效，应为：\n1：线圈\n2：视频\n3：地磁\n4：微波\n5：汽车电子标识（RFID）\n6：雷视一体\n9：其他'
        }
      }
      return { valid: true }
    }
  },
  position: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '检测位置不能为空' }
      }
      const validValues = [1, 2, 9]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '检测位置值无效，应为：\n1：进口\n2：出口\n9：其他'
        }
      }
      return { valid: true }
    }
  },
  target: {
    validate: (value) => {
      if (!value) return { valid: false, message: '检测对象不能为空' }
      if (!/^[01]{3}$/.test(value)) {
        return { valid: false, message: '检测对象格式错误，应为3位0或1的字符串，分别表示机动车、非机动车、行人的检测支持情况' }
      }
      return { valid: true }
    }
  },
  distance: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '距停车线距离不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 0) {
        return { valid: false, message: '距停车线距离必须是非负数，单位为厘米(cm)' }
      }
      return { valid: true }
    }
  },
  laneNoList: {
    validate: (value) => {
      if (!value?.laneNo?.length) {
        return { valid: false, message: '检测车道序号列表不能为空，至少包含1个车道序号' }
      }
      return { valid: true }
    }
  },
  pedestrianNoList: {
    validate: (value) => {
      // 可以为空
      if (!value) return { valid: true }
      if (!Array.isArray(value?.pedestrianNo)) {
        return { valid: false, message: '检测人行横道序号列表格式错误' }
      }
      return { valid: true }
    }
  }
}

// 添加车道参数的校验规则
const laneValidationRules = {
  crossID: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口编号不能为空' }
      const pattern = /^\d{12}$/
      if (!pattern.test(value)) {
        return { valid: false, message: '路口编号格式错误，应为12位数字' }
      }
      return { valid: true }
    }
  },
  laneNo: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '车道序号不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 1 || num > 99) {
        return { valid: false, message: '车道序号必须在1-99范围内' }
      }
      return { valid: true }
    }
  },
  direction: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '车道所在的进口方向不能为空' }
      }
      const validValues = [1, 2, 3, 4, 5, 6, 7, 8, 9]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '车道所在的进口方向值无效，应为：\n1：北\n2：东北\n3：东\n4：东南\n5：南\n6：西南\n7：西\n8：西北\n9：其他'
        }
      }
      return { valid: true }
    }
  },
  attribute: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '车道属性不能为空' }
      }
      const validValues = [0, 1, 2, 3, 9]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '车道属性值无效，应为：\n0：路口进口\n1：路口出口\n2：匝道\n3：路段车道\n9：其他'
        }
      }
      return { valid: true }
    }
  },
  movement: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '车道转向属性不能为空' }
      }
      const validValues = [11, 12, 13, 21, 22, 23, 24, 31, 32, 33, 34, 99]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '车道转向属性值无效，应为：\n11：直行\n12：左转\n13：右转\n21：直左混行\n22：直右混行\n23：左右混行\n24：直左右混行\n31：掉头\n32：掉头加左转\n33：掉头加直行\n34：掉头加右转\n99：其他'
        }
      }
      return { valid: true }
    }
  },
  feature: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '车道特性不能为空' }
      }
      const validValues = [1, 2, 3, 4, 9]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '车道特性值无效，应为：\n1：机动车车道\n2：非机动车车道\n3：机非混合车道\n4：行人便道\n9：其他'
        }
      }
      return { valid: true }
    }
  },
  azimuth: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '方位角不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 0 || num > 359) {
        return { valid: false, message: '方位角必须在0-359度范围内' }
      }
      return { valid: true }
    }
  },
  waitingArea: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '待行区不能为空' }
      }
      const validValues = [0, 1]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '待行区值无效，应为：\n0：无待行区\n1：有待行区'
        }
      }
      return { valid: true }
    }
  },
  varMovementList: {
    validate: (value) => {
      // 可以为空
      if (!value) return { valid: true }
      if (!Array.isArray(value?.movement) || value.movement.length < 2) {
        return { valid: false, message: '车道功能列表如果非空，则必须包含至少两个转向' }
      }
      return { valid: true }
    }
  }
}

// 添加人行横道参数的校验规则
const pedestrianValidationRules = {
  crossID: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口编号不能为空' }
      const pattern = /^\d{12}$/
      if (!pattern.test(value)) {
        return { valid: false, message: '路口编号格式错误，应为12位数字' }
      }
      return { valid: true }
    }
  },
  pedestrianNo: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '人行横道序号不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 1 || num > 99) {
        return { valid: false, message: '人行横道序号必须在1-99范围内' }
      }
      return { valid: true }
    }
  },
  direction: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '人行横道所在进口的方向不能为空' }
      }
      // 根据GB/T 39900—2021的A.18.3校验
      const validValues = [1, 2, 3, 4, 5, 6, 7, 8, 9]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '人行横道所在进口的方向值无效，应为：\n1：北\n2：东北\n3：东\n4：东南\n5：南\n6：西南\n7：西\n8：西北\n9：其他'
        }
      }
      return { valid: true }
    }
  },
  attribute: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '人行横道属性不能为空' }
      }
      const validValues = [1, 21, 22]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '人行横道属性值无效，应为：\n1：一次过街\n21：二次过街-路口进口\n22：二次过街-路口出口'
        }
      }
      return { valid: true }
    }
  }
}

// 添加信号组参数的校验规则
const signalGroupValidationRules = {
  crossID: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口编号不能为空' }
      const pattern = /^\d{12}$/
      if (!pattern.test(value)) {
        return { valid: false, message: '路口编号格式错误，应为12位数字' }
      }
      return { valid: true }
    }
  },
  signalGroupNo: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '信号组序号不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 1 || num > 99) {
        return { valid: false, message: '信号组序号必须在1-99范围内' }
      }
      return { valid: true }
    }
  },
  name: {
    validate: (value) => {
      if (!value) return { valid: false, message: '信号组名称不能为空' }
      if (value.length > 50) {
        return { valid: false, message: '信号组名称长度不能超过50' }
      }
      return { valid: true }
    }
  },
  greenFlashLen: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '绿闪时长不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 0) {
        return { valid: false, message: '绿闪时长必须是非负数，单位为秒(s)' }
      }
      return { valid: true }
    }
  },
  lampGroupNoList: {
    validate: (value) => {
      if (!value?.lampGroupNo?.length) {
        return { valid: false, message: '信号灯组序号列表不能为空，至少包含1个信号灯组序号' }
      }
      return { valid: true }
    }
  }
}

// 添加阶段参数的校验规则
const stageValidationRules = {
  crossID: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口编号不能为空' }
      const pattern = /^\d{12}$/
      if (!pattern.test(value)) {
        return { valid: false, message: '路口编号格式错误，应为12位数字' }
      }
      return { valid: true }
    }
  },
  stageNo: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '阶段号不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 0 || num > 9999) {
        return { valid: false, message: '阶段号必须在0-9999范围内' }
      }
      return { valid: true }
    }
  },
  stageName: {
    validate: (value) => {
      if (!value) return { valid: false, message: '阶段名称不能为空' }
      if (value.length > 50) {
        return { valid: false, message: '阶段名称长度不能超过50' }
      }
      return { valid: true }
    }
  },
  attribute: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '特征不能为空' }
      }
      const validValues = [0, 1]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '特征值无效，应为：\n0：一般\n1：感应'
        }
      }
      return { valid: true }
    }
  },
  signalGroupStatusList: {
    validate: (value) => {
      if (!value?.signalGroupStatus?.length) {
        return { valid: false, message: '信号组灯色状态列表不能为空，至少包含1个信号组灯色状态' }
      }
      // 校验每个信号组灯色状态
      for (const status of value.signalGroupStatus) {
        if (!status.signalGroupNo || status.signalGroupNo < 1 || status.signalGroupNo > 99) {
          return { valid: false, message: '信号组序号必须在1-99范围内' }
        }
        if (!status.lampStatus || !/^[0-3]{3}$/.test(status.lampStatus)) {
          return { 
            valid: false, 
            message: '灯色状态格式错误，应为3位0-3的字符串，分别表示：\n' +
                    '有轨电车信号灯组(61/62/63)：禁止通行、过渡、通行信号灯色状态\n' +
                    '其他信号灯组：红灯、黄灯、绿灯色状态\n' +
                    '每位字符取值：\n' +
                    '0：无灯\n' +
                    '1：灭灯\n' +
                    '2：亮灯\n' +
                    '3：闪灯'
          }
        }
      }
      return { valid: true }
    }
  }
}

// 添加配时方案参数的校验规则
const planValidationRules = {
  crossID: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口编号不能为空' }
      const pattern = /^\d{12}$/
      if (!pattern.test(value)) {
        return { valid: false, message: '路口编号格式错误，应为12位数字' }
      }
      return { valid: true }
    }
  },
  planNo: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '方案序号不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 1 || num > 9999) {
        return { valid: false, message: '方案序号必须在1-9999范围内' }
      }
      return { valid: true }
    }
  },
  planName: {
    validate: (value) => {
      if (!value) return { valid: false, message: '方案名称不能为空' }
      if (value.length > 50) {
        return { valid: false, message: '方案名称长度不能超过50' }
      }
      return { valid: true }
    }
  },
  cycleLen: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '周期长度不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num <= 0) {
        return { valid: false, message: '周期长度必须大于0，单位为秒(s)' }
      }
      return { valid: true }
    }
  },
  coordStageNo: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '协调相位号不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 0) {
        return { valid: false, message: '协调相位号必须大于等于0' }
      }
      return { valid: true }
    }
  },
  offset: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '协调相位差不能为空' }
      }
      const num = Number(value)
      if (isNaN(num)) {
        return { valid: false, message: '协调相位差必须是数字，单位为秒(s)' }
      }
      return { valid: true }
    }
  },
  stageTimingList: {
    validate: (value) => {
      if (!value?.stageTiming?.length) {
        return { valid: false, message: '阶段配时信息列表不能为空，至少包含1个阶段配时信息' }
      }
      
      // 校验每个阶段配时信息
      for (const timing of value.stageTiming) {
        // 1. 校验阶段号
        if (timing.stageNo === undefined || timing.stageNo === null) {
          return { valid: false, message: '阶段号不能为空' }
        }
        const stageNo = Number(timing.stageNo)
        if (isNaN(stageNo) || stageNo < 0 || stageNo > 9999) {
          return { valid: false, message: '阶段号必须在0-9999范围内' }
        }

        // 2. 校验绿灯时长
        if (timing.green === undefined || timing.green === null) {
          return { valid: false, message: '绿灯时长不能为空' }
        }
        const green = Number(timing.green)
        if (isNaN(green) || green <= 0) {
          return { valid: false, message: '绿灯时长必须大于0，单位为秒(s)' }
        }

        // 3. 校验黄灯时长
        if (timing.yellow === undefined || timing.yellow === null) {
          return { valid: false, message: '黄灯时长不能为空' }
        }
        const yellow = Number(timing.yellow)
        if (isNaN(yellow) || yellow < 0) {
          return { valid: false, message: '黄灯时长必须大于等于0，单位为秒(s)' }
        }

        // 4. 校验全红时长
        if (timing.allRed === undefined || timing.allRed === null) {
          return { valid: false, message: '全红时长不能为空' }
        }
        const allRed = Number(timing.allRed)
        if (isNaN(allRed) || allRed < 0) {
          return { valid: false, message: '全红时长必须大于等于0，单位为秒(s)' }
        }

        // 7. 校验迟开早闭配置列表（可选）
        if (timing.adjustList?.adjust) {
          for (const adjust of timing.adjustList.adjust) {
            // 7.1 校验信号组序号
            if (adjust.signalGroupNo === undefined || adjust.signalGroupNo === null) {
              return { valid: false, message: '信号组序号不能为空' }
            }
            const signalGroupNo = Number(adjust.signalGroupNo)
            if (isNaN(signalGroupNo) || signalGroupNo < 1 || signalGroupNo > 99) {
              return { valid: false, message: '信号组序号必须在1-99范围内' }
            }

            // 7.2 校验调整方式
            if (adjust.oper === undefined || adjust.oper === null) {
              return { valid: false, message: '调整方式不能为空' }
            }
            const oper = Number(adjust.oper)
            if (isNaN(oper) || ![1, 2].includes(oper)) {
              return { valid: false, message: '调整方式值无效，应为：\n1：迟开\n2：早闭' }
            }

            // 7.3 校验调整时间
            if (adjust.len === undefined || adjust.len === null) {
              return { valid: false, message: '调整时间不能为空' }
            }
            const len = Number(adjust.len)
            if (isNaN(len) || len <= 0) {
              return { valid: false, message: '调整时间必须大于0，单位为秒(s)' }
            }
          }
        }
      }
      return { valid: true }
    }
  },
  // ... existing code ...
}

// 添加日计划参数的校验规则
const dayPlanValidationRules = {
  crossID: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口编号不能为空' }
      const pattern = /^\d{12}$/
      if (!pattern.test(value)) {
        return { valid: false, message: '路口编号格式错误，应为12位数字' }
      }
      return { valid: true }
    }
  },
  dayPlanNo: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '日计划号不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 1 || num > 999) {
        return { valid: false, message: '日计划号必须在1-999范围内' }
      }
      return { valid: true }
    }
  },
  periodList: {
    validate: (value) => {
      if (!value?.period?.length) {
        return { valid: false, message: '时段信息列表不能为空，至少包含1个时段' }
      }
      // 校验每个时段信息
      for (const period of value.period) {
        if (!period.startTime || !/^([01]\d|2[0-3]):[0-5]\d$/.test(period.startTime)) {
          return { valid: false, message: '开始时间格式错误，应为hh24:mm格式' }
        }
        if (!period.planNo || period.planNo < 1 || period.planNo > 9999) {
          return { valid: false, message: '配时方案序号必须在1-9999范围内' }
        }
        if (!period.ctrlMode || !/^(0[01]|1[1-3]|2[1-3]|3[1]|4[1]|5[1-3])$/.test(period.ctrlMode)) {
          return { 
            valid: false, 
            message: '控制方式值无效，应为：\n00：撤销或恢复自主\n01：本地手动控制\n11：特殊控制-全部关灯\n12：特殊控制-全红\n13：特殊控制-全部黄闪\n21：单点多时段定时控制\n22：单点感应控制\n23：单点自适应控制\n31：线协调定时控制\n32：线协调感应控制\n33：线协调自适应控制\n41：区域协调控制\n51：干预控制-手动控制\n52：干预控制-锁定阶段\n53：干预控制-指定方案'
          }
        }
      }
      return { valid: true }
    }
  }
}

// 添加调度参数的校验规则
const scheduleValidationRules = {
  crossID: {
    validate: (value) => {
      if (!value) return { valid: false, message: '路口编号不能为空' }
      const pattern = /^\d{12}$/
      if (!pattern.test(value)) {
        return { valid: false, message: '路口编号格式错误，应为12位数字' }
      }
      return { valid: true }
    }
  },
  scheduleNo: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '调度号不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 1 || num > 999) {
        return { valid: false, message: '调度号必须在1-999范围内' }
      }
      return { valid: true }
    }
  },
  type: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '调度类型不能为空' }
      }
      const validValues = [1, 2, 3]
      if (!validValues.includes(Number(value))) {
        return { 
          valid: false, 
          message: '调度类型值无效，应为：\n1：特殊日调度\n2：时间段周调度\n3：周调度'
        }
      }
      return { valid: true }
    }
  },
  startDay: {
    validate: (value, context) => {
      // 如果是周调度（type=3），跳过校验
      if (context?.type === 3) return { valid: true }
      if (!value) return { valid: false, message: '开始月日不能为空' }
      if (!/^(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])$/.test(value)) {
        return { valid: false, message: '开始月日格式错误，应为MM-DD格式' }
      }
      return { valid: true }
    }
  },
  endDay: {
    validate: (value, context) => {
      // 如果是周调度（type=3），跳过校验
      if (context?.type === 3) return { valid: true }
      if (!value) return { valid: false, message: '结束月日不能为空' }
      if (!/^(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])$/.test(value)) {
        return { valid: false, message: '结束月日格式错误，应为MM-DD格式' }
      }
      return { valid: true }
    }
  },
  weekDay: {
    validate: (value, context) => {
      if (context?.type === 1) return { valid: true } // 特殊日调度时无意义
      if (value === undefined || value === null) {
        return { valid: false, message: '周几不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 1 || num > 7) {
        return { valid: false, message: '周几必须在1-7范围内，分别代表周一至周日' }
      }
      return { valid: true }
    }
  },
  dayPlanNo: {
    validate: (value) => {
      if (value === undefined || value === null) {
        return { valid: false, message: '日计划号不能为空' }
      }
      const num = Number(value)
      if (isNaN(num) || num < 1 || num > 999) {
        return { valid: false, message: '日计划号必须在1-999范围内' }
      }
      return { valid: true }
    }
  }
}

// 获取字段的校验结果
const getFieldValidation = (field, value, context = 'cross') => {
  let rules
  switch (context) {
    case 'detector':
      rules = detectorValidationRules
      break
    case 'lane':
      rules = laneValidationRules
      break
    case 'pedestrian':
      rules = pedestrianValidationRules
      break
    case 'signalGroup':
      rules = signalGroupValidationRules
      break
    case 'stage':
      rules = stageValidationRules
      break
    case 'plan':
      rules = planValidationRules
      break
    case 'dayPlan':
      rules = dayPlanValidationRules
      break
    case 'schedule':
      rules = scheduleValidationRules
      // 如果是调度参数，传入调度类型作为上下文
      if (typeof value === 'object' && value !== null) {
        return rules[field]?.validate(value, { type: value.type }) || { valid: true }
      }
      break
    case 'lampGroup':
      rules = lampGroupValidationRules
      break
    default:
      rules = validationRules
  }
  
  const rule = rules[field]
  if (!rule) return { valid: true }
  
  // 对于需要上下文的校验规则，传入上下文
  if (rule.validate.length > 1) {
    return rule.validate(value, { type: value?.type })
  }
  return rule.validate(value)
}

// 复制JSON到剪贴板
const copyJson = async (type) => {
  try {
    const jsonStr = type === 'schedules' ? getFilteredRawJsonData(type) : getRawJsonData(type)
    await navigator.clipboard.writeText(jsonStr)
    ElMessage.success('已复制到剪贴板')
  } catch (error) {
    ElMessage.error('复制失败：' + error.toString())
  }
}

// 搜索键
const handleSearch = async () => {
  try {
    const pattern = searchPattern.value.trim() 
      ? `1049Cache:param:CrossParam:${searchPattern.value}`
      : '1049Cache:param:CrossParam:*'
      
    const response = await invoke('search_keys', {
      pattern
    })
    
    if (!response.success) {
      throw new Error(response.error || '搜索失败')
    }

    if (!response.data) {
      keyTree.value = []
      return
    }
    
    // 构建树形结构
    const keys = response.data
    keyTree.value = keys.map(key => ({
      key,
      name: key.split(':').pop(),
      children: []
    }))
  } catch (error) {
    ElMessage.error('搜索键失败：' + error.toString())
  }
}

// 点击键
const handleNodeClick = async (data) => {
  selectedKey.value = data.key
  await loadKeyData()
}

// 加载键数据
const loadKeyData = async () => {
  if (!selectedKey.value) return
  
  try {
    const response = await invoke('get_key_data', {
      key: selectedKey.value
    })
    
    if (!response.success) {
      throw new Error(response.error || '获取数据失败')
    }

    if (!response.data) {
      throw new Error('未获取到数据')
    }

    const data = response.data
    keyType.value = data.type
    ttl.value = data.ttl
    
    // 解析 JSON 字符串
    if (typeof data.value === 'string') {
      try {
        value.value = JSON.parse(data.value)
      } catch (e) {
        console.error('解析JSON失败:', e)
        value.value = data.value
      }
    } else {
      value.value = data.value
    }
  } catch (error) {
    ElMessage.error('获取键数据失败：' + error.toString())
  }
}

// 刷新值
const refreshValue = () => {
  loadKeyData()
}

// 删除键
const deleteKey = async () => {
  try {
    await ElMessageBox.confirm(
      `确定要删除键 "${selectedKey.value}" 吗？`,
      '警告',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    await invoke('delete_key', {
      key: selectedKey.value
    })
    
    ElMessage.success('删除成功')
    selectedKey.value = ''
    handleSearch()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除键失败：' + error.toString())
    }
  }
}

// 格式化值显示
const formattedValue = computed(() => {
  if (!value.value) return ''
  
  try {
    // 如果已经是对象，直接格式化
    if (typeof value.value === 'object') {
      return JSON.stringify(value.value, null, 2)
    }
    // 如果是字符串，尝试解析后格式化
    return JSON.stringify(JSON.parse(value.value), null, 2)
  } catch {
    // 如果解析失败，直接返回原值
    return value.value
  }
})

// 获取详细信息
const fetchDetail = async (type, no, crossId) => {
  const key = `1049Cache:param:${type}:${crossId}:${no}`
 
  try {
    const response = await invoke('get_key_data', { key })
    if (!response.success || !response.data) {
      throw new Error(response.error || '获取数据失败')
    }
    // 保存原始 JSON 字符串
    const rawValue = response.data.value
    console.log(key,rawValue)
    // 保存到原始 JSON 数据中
    const typeKey = type === 'LampGroup' ? 'lampGroups' :
                   type === 'LaneParam' ? 'lanes' :
                   type === 'PedestrianParam' ? 'pedestrians' :
                   type === 'StageParam' ? 'stages' :
                   type === 'DetParam' ? 'detectors' :
                   type === 'PlanParam' ? 'plans' :
                   type === 'SignalGroupParam' ? 'signalGroups' :
                   type === 'ScheduleParam' ? 'schedules' :
                   type === 'DayPlanParam' ? 'dayPlans' : null
                  
    if (typeKey) {
      rawJsonData.value[typeKey][no] = rawValue
    }
    
    // 解析 JSON 字符串用于表格视图
    if (typeof rawValue === 'string') {
      try {
        return JSON.parse(rawValue)
      } catch (e) {
        console.error(`解析${type} ${no}的JSON失败:`, e)
        return rawValue
      }
    }
    return rawValue
  } catch (error) {
    console.error(`获取${type} ${no}失败:`, error)
    return null
  }
}

// 修改 JSON 视图的显示
const getRawJsonData = (type) => {
  const typeKey = type === 'lampGroups' ? 'lampGroups' :
                 type === 'lanes' ? 'lanes' :
                 type === 'pedestrians' ? 'pedestrians' :
                 type === 'stages' ? 'stages' :
                 type === 'detectors' ? 'detectors' :
                 type === 'plans' ? 'plans' :
                 type === 'signalGroups' ? 'signalGroups' :
                 type === 'schedules' ? 'schedules' : 
                 type === 'dayPlans' ? 'dayPlans' :''

  if (!typeKey) return '[]'
  
  // 将对象转换为数组
  const data = Object.values(rawJsonData.value[typeKey])
  // 尝试解析每个 JSON 字符串
  return JSON.stringify(data.map(item => {
    try {
      return typeof item === 'string' ? JSON.parse(item) : item
    } catch {
      return item
    }
  }), null, 2)
}

// 加载所有详细信息
const loadAllDetails = async () => {
  if (!value.value?.crossID) return

  const crossId = value.value.crossID

  // 清空所有原有信息
  lampGroupDetails.value = {}
  laneDetails.value = {}
  pedestrianDetails.value = {}
  stageDetails.value = {}
  detDetails.value = {}
  planDetails.value = {}
  signalGroupDetails.value = {}
  scheduleDetails.value = {}
 dayPlanDetails.value = {}

  // 加载灯组信息
  for (const no of lampGroupNos.value) {
      lampGroupDetails.value[no] = await fetchDetail('LampGroup', no, crossId)
  }

  // 加载车道信息
  for (const no of laneNos.value) {
      laneDetails.value[no] = await fetchDetail('LaneParam', no, crossId)
  }

  // 加载行人信息
  for (const no of pedestrianNos.value) {
      pedestrianDetails.value[no] = await fetchDetail('PedestrianParam', no, crossId)
  }

  // 加载相位信息
  for (const no of stageNos.value) {
      stageDetails.value[no] = await fetchDetail('StageParam', no, crossId)
  }

  // 加载检测器信息
  for (const no of detNos.value) {
      detDetails.value[no] = await fetchDetail('DetParam', no, crossId)
  }

  // 加载方案信息
  for (const no of planNos.value) {
      planDetails.value[no] = await fetchDetail('PlanParam', no, crossId)
  }

  // 加载信号组信息
  for (const no of signalGroupNos.value) {
      signalGroupDetails.value[no] = await fetchDetail('SignalGroupParam', no, crossId)
  }

  // 加载调度信息
  for (const no of scheduleNos.value) {
      scheduleDetails.value[no] = await fetchDetail('ScheduleParam', no, crossId)
  }
  // 加载日计划信息
  for (const no of dayPlanNos.value) {
    dayPlanDetails.value[no] = await fetchDetail('DayPlanParam', no, crossId)
  }
  console.log( dayPlanDetails.value)
  Object.values(stageDetails.value).forEach(stage => {
      const cross = {
      stageParamList: Object.values(stageDetails.value),
      signalGroupParamList: Object.values(signalGroupDetails.value),
      lampGroupList: Object.values(lampGroupDetails.value)
    }
    let temp = getPhaseInfo(cross,stage.stageNo)
    
    stageDetails.value[stage.stageNo].phaseInfo = temp
  })


  
 
 

 
}

// 监听值变化，加载详细信息
watch(value, async (newValue) => {
  if (!newValue) return
  
  // 确保 newValue 是对象
  const valueObj = typeof newValue === 'string' ? 
    (() => {
      try {
        return JSON.parse(newValue)
      } catch (e) {
        console.error('解析JSON失败:', e)
        return null
      }
    })() : 
    newValue

  if (valueObj?.crossID) {
    await loadAllDetails()
    await loadRunInfo()
  }
}, { deep: true })

// 格式化标签显示
const formatLabel = (key) => {
  const labelMap = {
    // 可以根据需要添加更多映射
  }
  return labelMap[key] || key
}

// 添加字典映射
const dictMaps = {
  controlMode: {
    '00': '撤销或恢复自主',
    '01': '本地手动控制',
    '11': '特殊控制-全部关灯',
    '12': '特殊控制-全红',
    '13': '特殊控制-全部黄闪',
    '21': '单点多时段定时控制',
    '22': '单点感应控制',
    '23': '单点自适应控制',
    '31': '线协调定时控制',
    '32': '线协调感应控制',
    '33': '线协调自适应控制',
    '41': '区域协调控制',
    '51': '干预控制-手动控制',
    '52': '干预控制-锁定阶段',
    '53': '干预控制-指定方案'
  },
  crossFeature: {
    '10': '行人过街',
    '12': '2次行人过街',
    '23': 'T形、Y形',
    '24': '十字形',
    '35': '五岔路口',
    '36': '六岔路口',
    '39': '多岔路口',
    '40': '环形交叉口(环岛)',
    '50': '匝道',
    '51': '匝道-入口',
    '52': '匝道-出口',
    '61': '快速路主路路段(交汇区)',
    '90': '其他'
  },
  direction: {
    '1': '北',
    '2': '东北',
    '3': '东',
    '4': '东南',
    '5': '南',
    '6': '西南',
    '7': '西',
    '8': '西北',
    '9': '其他'
  },
  pedsAttr: {
    '1': '一次过街',
    '21': '二次过街-交叉口进口',
    '22': '二次过街-交叉口出口'
  },
  laneMovement: {
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
  },
  laneAttribute: {
    '0': '路口进口',
    '1': '路口出口',
    '2': '匝道',
    '3': '路段车道',
    '9': '其他'
  },
  laneFeature: {
    '1': '机动车道',
    '2': '非机动车道',
    '3': '机非混合车道',
    '4': '行人便道',
    '9': '其他'
  },
  lampGroup: {
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
  },
  crossGrade: {
    '11': '一级,主干路与主干路相交交叉口',
    '12': '二级,主干路与次干路相交交叉口',
    '13': '三级,主干路与支路相交交叉口',
    '21': '四级,次干路与次干路相交交叉口',
    '22': '五级,次干路与支路相交交叉口',
    '31': '六级,支路与支路相交交叉口',
    '99': '其他'
  },
  routeType: {
    '1': '协调干线',
    '2': '公交优先线路',
    '3': '特勤线路',
    '4': '有轨电车线路',
    '5': '快速路(沿线匝道路口)',
    '9': '其他'
  },
  detType: {
    '1': '线圈',
    '2': '视频',
    '3': '地磁',
    '4': '微波',
    '5': '汽车电子标识(RFID)',
    '6': '雷视一体',
    '9': '其他'
  },
  detPosition: {
    '1': '进口',
    '2': '出口',
    '9': '其他'
  },
  detTarget: {
    '100': '机动车',
    '010': '非机动车',
    '001': '行人',
    '011': '非机动车+行人'
  },
  // 添加灯色状态字典映射
  lampStatus: {
    '0': '无灯',
    '1': '灭灯',
    '2': '亮灯',
    '3': '闪灯'
  },
  stageTiming: {
    stageNo: '阶段号',
    green: '绿灯时长(s)',
    yellow: '黄灯时长(s)',
    allRed: '全红时长(s)',
    maxGreen: '感应/自适应控制最大绿灯时长(s)',
    minGreen: '感应/自适应控制最小绿灯时长(s)',
    adjustList: '迟开早闭配置列表',
    signalGroupNo: '信号组序号',
    oper: '调整方式',
    len: '调整时间(s)'
  },
  scheduleType: {
    '1': '特殊日调度',
    '2': '时间段周调度',
    '3': '周调度'
  },
  weekDay: {
    '1': '周一',
    '2': '周二',
    '3': '周三',
    '4': '周四',
    '5': '周五',
    '6': '周六',
    '7': '周日'
  }
}

// 添加详细信息视图模式状态
const detailViewModes = ref({
  lampGroups: 'table',
  lanes: 'table',
  pedestrians: 'table',
  stages: 'table',
  detectors: 'table',
  plans: 'table',
  signalGroups: 'table',
  schedules: 'table',
  dayPlans: 'table',
})

// 修改格式化值的函数，增加灯色状态的特殊处理
const formatValue = (val, field, context = 'cross') => {
  if (val === null) return '<span class="type-null">null</span>'
  if (val === undefined) return '<span class="type-undefined">undefined</span>'
  
  const type = typeof val
  let displayValue = val
  
  // 处理时间戳
  if (field === 'recordTime' && typeof val === 'number') {
    const date = new Date(val)
    displayValue = val+'-'+date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false
    })
  }
  // 处理灯色状态
  else if (field === 'lampStatus') {
    if (typeof val === 'string' && val.length === 3) {
      const statusMap = {
        '0': '无灯',
        '1': '灭灯',
        '2': '亮灯',
        '3': '闪灯'
      }
      // 获取信号组类型，用于判断是有轨电车信号还是普通信号
      const signalGroupType = context?.signalGroupType
      const isTramSignal = ['61', '62', '63'].includes(signalGroupType)
      
      const statusDesc = isTramSignal ? 
        ['禁止通行', '过渡', '通行'] : 
        ['红灯', '黄灯', '绿灯']
      
      const statusDetails = val.split('').map((s, i) => 
        `${statusDesc[i]}: ${statusMap[s] || '未知'}`
      ).join('\n')
      
      displayValue = `${val}\n${statusDetails}`
    }
  } else {
    // 原有的字典映射处理逻辑
    let dict = null
    if (field === 'direction') {
      dict = dictMaps.direction
    } else if (field === 'feature' && context === 'cross') {
      dict = dictMaps.crossFeature
    } else if (field === 'grade') {
      dict = dictMaps.crossGrade
    } else if (field === 'type' && context === 'detector') {
      dict = dictMaps.detType
    } else if (field === 'position' && context === 'detector') {
      dict = dictMaps.detPosition
    } else if (field === 'target' && context === 'detector') {
      dict = dictMaps.detTarget
    } else if (field === 'movement' && context === 'lane') {
      dict = dictMaps.laneMovement
    } else if (field === 'attribute' && context === 'lane') {
      dict = dictMaps.laneAttribute
    } else if (field === 'feature' && context === 'lane') {
      dict = dictMaps.laneFeature
    } else if (field === 'attribute' && context === 'pedestrian') {
      dict = dictMaps.pedsAttr
    } else if (field === 'type' && context === 'lampGroup') {
      dict = dictMaps.lampGroup
    } else if (field === 'ctrlMode') {
      dict = dictMaps.controlMode
    } else if (field === 'type' && (context === 'schedule' || (typeof context === 'object' && context?.type !== undefined))) {
      dict = dictMaps.scheduleType
    } else if (field === 'weekDay' && (context === 'schedule' || (typeof context === 'object' && context?.type !== undefined))) {
      dict = dictMaps.weekDay
    }

    if (dict && dict[val]) {
      displayValue = `${val} (${dict[val]})`
    } else if (type === 'boolean') {
      displayValue = val ? '是' : '否'
    } else if (type === 'object') {
      if (Array.isArray(val)) {
        displayValue = JSON.stringify(val, null, 2)
      } else {
        displayValue = JSON.stringify(val, null, 2)
      }
    } else if (type === 'number') {
      displayValue = val.toString()
    } else if (type === 'string') {
      try {
        const parsed = JSON.parse(val)
        displayValue = JSON.stringify(parsed, null, 2)
      } catch {
        displayValue = val
      }
    }
  }

  // 添加校验结果
  const validation = field ? getFieldValidation(field, val, context) : { valid: true }
  const validationClass = validation.valid ? '' : 'invalid-value'
  const validationMessage = validation.valid ? '' : `<div class="validation-message">${validation.message}</div>`

  return `<div class="value-with-type ${validationClass}">
    <span class="type-tag type-${type}">${type}</span>
    <span class="value-content">${displayValue}</span>
    ${validationMessage}
  </div>`
}

// 修改阶段信息的显示逻辑，传递信号组类型信息
const formatStageValue = (val, field, stageData) => {
  if (field === 'lampStatus' && stageData?.signalGroupNo) {
    // 获取信号组类型
    const signalGroupType = signalGroupDetails.value[stageData.signalGroupNo]?.type
    return formatValue(val, field, { signalGroupType })
  }
  return formatValue(val, field, 'stage')
}

onMounted(() => {
  handleSearch()
})

// 在 script setup 部分添加
const mapSize = ref(800) // 路口图大小

// 根据方向值获取九宫格的 row 和 col
const getGridPosition = (direction, attribute) => {
  // attribute: 0:路口进口, 1:路口出口
  const isImport = attribute === 0;

  switch (direction) {
    case 1: // 北
      return isImport ? { row: 3, col: 1 } : { row: 0, col: 2 };
    case 2: return { row: isImport ? 3 : 0, col: isImport ? 2 : 3 }; // 东北 (简化处理)
    case 3: // 东
      return isImport ? { row: 1, col: 0 } : { row: 2, col: 3 };
    case 4: return { row: isImport ? 0 : 3, col: isImport ? 0 : 1 }; // 东南 (简化处理)
    case 5: // 南
      return isImport ? { row: 0, col: 2 } : { row: 3, col: 1 };
    case 6: return { row: isImport ? 0 : 3, col: isImport ? 1 : 0 }; // 西南 (简化处理)
    case 7: // 西
      return isImport ? { row: 2, col: 3 } : { row: 1, col: 0 };
    case 8: return { row: isImport ? 3 : 0, col: isImport ? 3 : 0 }; // 西北 (简化处理)
    case 9: // 其他/中心
      return { row: 1, col: 1 }; // 中心区域不变
    default:
      return { row: 1, col: 1 }; // 默认为中心
  }
}

// 将车道按方向分组
const lanesByDirectionAndAttribute = computed(() => {
  const groups = {};
  if (laneDetails.value) {
    Object.values(laneDetails.value).forEach(lane => {
      const key = `${lane.direction}-${lane.attribute}`;
      if (!groups[key]) {
        groups[key] = [];
      }
      groups[key].push(lane);
    });
  }
  return groups;
});

// 添加格式化转向属性的函数
const formatMovement = (movement) => {
  const movementMap = {
    '11': '直',
    '12': '左',
    '13': '右',
    '21': '直左',
    '22': '直右',
    '23': '左右',
    '24': '直左右',
    '31': '掉头',
    '32': '掉左',
    '33': '掉直',
    '34': '掉右',
    '99': '其他'
  };
  // 检查 movement 是否为 undefined 或 null，或者不在映射中
  if (movement === undefined || movement === null || !movementMap[movement.toString()]) {
      return '未知';
}
  return movementMap[movement.toString()] || movement;
};

// 添加原始 JSON 数据的响应式引用
const rawJsonData = ref({
  lampGroups: {},
  lanes: {},
  pedestrians: {},
  stages: {},
  detectors: {},
  plans: {},
  signalGroups: {},
  schedules: {},
  dayPlans:{}
})

// 在 script setup 部分添加过滤相关的状态
const scheduleTypeFilter = ref('all') // 添加调度类型过滤状态

// 添加过滤后的调度号列表计算属性
const filteredScheduleNos = computed(() => {
  if (scheduleTypeFilter.value === 'all') {
    return scheduleNos.value
  }
  return scheduleNos.value.filter(no => {
    const type = scheduleDetails.value[no]?.type
    return type && type.toString() === scheduleTypeFilter.value
  })
})

// 添加过滤后的 JSON 数据获取函数
const getFilteredRawJsonData = (type) => {
  if (type !== 'schedules') {
    return getRawJsonData(type)
  }
  
  // 获取所有调度数据
  const allData = Object.entries(rawJsonData.value.schedules)
    .filter(([no]) => filteredScheduleNos.value.includes(Number(no)))
    .map(([, value]) => {
      try {
        return typeof value === 'string' ? JSON.parse(value) : value
      } catch {
        return value
      }
    })
  
  return JSON.stringify(allData, null, 2)
}

// 在 script setup 部分添加列数选择相关的状态
const detailColumns = ref({
  lampGroups: 4,
  lanes: 4,
  pedestrians: 4,
  stages: 4,
  detectors: 4,
  plans: 4,
  signalGroups: 4,
  schedules: 4,
  dayPlans:3
})

// 添加获取指定方向和属性的车道列表的方法
const getLanesByDirectionAndAttribute = (direction, attribute) => {
  const lanes = Object.values(laneDetails.value).filter(lane => 
    lane.direction === direction && lane.attribute === attribute
  );
  
  // 根据方向调整排序方式
  switch (direction) {
    case 1: // 北向
    return lanes.sort((a, b) => b.laneNo - a.laneNo);
    case 5: // 南向
      // 从左到右排序（车道号从小到大）
      return lanes.sort((a, b) => a.laneNo - b.laneNo);
    case 3: // 东向
    return lanes.sort((a, b) => b.laneNo - a.laneNo);
    case 7: // 西向
      // 从上到下排序（车道号从小到大）
      return lanes.sort((a, b) => a.laneNo - b.laneNo);
   
    default:
      return lanes.sort((a, b) => a.laneNo - b.laneNo);
  }
};

// 添加获取车道提示信息的方法
const getLaneTooltip = (lane) => {
  const directionMap = {
    1: '北',
    3: '东',
    5: '南',
    7: '西'
  };
  const attributeMap = {
    0: '进口',
    1: '出口'
  };
  const movementMap = {
    11: '直行',
    12: '左转',
    13: '右转',
    21: '直左混行',
    22: '直右混行',
    23: '左右混行',
    24: '直左右混行',
    31: '掉头',
    32: '掉头加左转',
    33: '掉头加直行',
    34: '掉头加右转',
    99: '其他'
  };
  const featureMap = {
    1: '机动车道',
    2: '非机动车道',
    3: '机非混合车道',
    4: '行人便道',
    9: '其他'
  };

  return `车道号: ${lane.laneNo}
方向: ${directionMap[lane.direction] || '未知'}
类型: ${attributeMap[lane.attribute] || '未知'}
转向: ${movementMap[lane.movement] || '未知'}
特性: ${featureMap[lane.feature] || '未知'}`;
};

// 添加获取指定方向灯组的方法
const getLampGroupsByDirection = (direction) => {
  if (!lampGroupDetails.value) return [];
  // 只包含特定类型的机动车信号灯 (10, 21, 22, 23)
  const targetLampTypes = ['10', '21', '22', '23'];
  return Object.values(lampGroupDetails.value).filter(lampGroup => 
    lampGroup.direction == direction && targetLampTypes.includes(lampGroup.type?.toString())
  );
};

// 添加获取灯组提示信息的方法 (可选)
const getLampGroupTooltip = (lampGroup) => {
  const directionMap = {
    1: '北', 3: '东', 5: '南', 7: '西'
    // 可以根据需要添加更多方向映射
  };
   const typeMap = {
    '10': '机动车信号灯 (圆盘)',
    '21': '直行方向指示灯',
    '22': '左转方向指示灯',
    '23': '右转方向指示灯',
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
   };
  return `灯组号: ${lampGroup.lampGroupNo}\n方向: ${directionMap[lampGroup.direction] || '未知'}\n类型: ${typeMap[lampGroup.type] || '未知'}`;
};

// 路口运行信息
const runInfo = ref({
  crossCtrlInfo: null,
  cycleInfo: null,
  stageInfo: null,
  crossSignalGroupStatus: null,
  phaseInfo: null,
  planInfo: null
})

const loadRunInfo = async () => {
  runInfo.value = {
    crossCtrlInfo: null,
    cycleInfo: null,
    stageInfo: null,
    crossSignalGroupStatus: null,
    phaseInfo: null,
    planInfo: null
  }
  if (!value.value?.crossID) {
    runInfo.value = {
      crossCtrlInfo: null,
      cycleInfo: null,
      stageInfo: null,
      crossSignalGroupStatus: null,
      phaseInfo: null,
      planInfo: null
    }
    return
  }

  try {
    // 加载控制信息
    const ctrlKey = `1049Cache:runInfo:CrossCtrlInfo:${value.value.crossID}`
    const response = await invoke('get_key_data', { key: ctrlKey })
    if (response.success && response.data && response.data.value) {
      runInfo.value.crossCtrlInfo = typeof response.data.value === 'string'
        ? JSON.parse(response.data.value)
        : response.data.value
    } else {
      runInfo.value.crossCtrlInfo = null
    }
    // 根据控制信息中的方案号查找对应方案
    if (runInfo.value.crossCtrlInfo?.planNo) {
      const planNo = runInfo.value.crossCtrlInfo.planNo
      const planDetail = planDetails.value[planNo]
      if (planDetail) {
        runInfo.value.planInfo = planDetail
      }
    }

    // 加载周期信息
    const cycleKey = `1049Cache:runInfo:CrossCycle:${value.value.crossID}`
    const cycleResponse = await invoke('get_key_data', { key: cycleKey })
    if (cycleResponse.success && cycleResponse.data && cycleResponse.data.value) {
      runInfo.value.cycleInfo = typeof cycleResponse.data.value === 'string'
        ? JSON.parse(cycleResponse.data.value)
        : cycleResponse.data.value
    }

    // 加载阶段信息
    const stageKey = `1049Cache:runInfo:CrossStage:${value.value.crossID}`
    const stageResponse = await invoke('get_key_data', { key: stageKey })
    if (stageResponse.success && stageResponse.data && stageResponse.data.value) {
      runInfo.value.stageInfo = typeof stageResponse.data.value === 'string'
        ? JSON.parse(stageResponse.data.value)
        : stageResponse.data.value
    }
    if (runInfo.value.stageInfo?.curStageNo) {
    
      const currentStage = stageDetails.value[runInfo.value.stageInfo.curStageNo]
      if(currentStage){
        runInfo.value.phaseInfo=currentStage.phaseInfo
        console.log(runInfo.value.phaseInfo)
      }
    }

    // 加载信号组状态信息
    const crossSignalGroupStatusKey = `1049Cache:runInfo:CrossSignalGroupStatus:${value.value.crossID}`
    const signalGroupResponse = await invoke('get_key_data', { key: crossSignalGroupStatusKey })
    if (signalGroupResponse.success && signalGroupResponse.data && signalGroupResponse.data.value) {
      runInfo.value.crossSignalGroupStatus = typeof signalGroupResponse.data.value === 'string'
        ? JSON.parse(signalGroupResponse.data.value)
        : signalGroupResponse.data.value
    }

    if (runInfo.value.crossSignalGroupStatus?.signalGroupStatusList?.signalGroupStatus) {
      runInfo.value.crossSignalGroupStatus.signalGroupStatusList.signalGroupStatus.forEach(status => {
        const signalGroupDetail = signalGroupDetails.value[status.signalGroupNo]
        if (signalGroupDetail?.lampGroupNoList?.lampGroupNo) {
          const lampGroupNos = signalGroupDetail.lampGroupNoList.lampGroupNo
          lampGroupNos.forEach(no => {
            if (lampGroupDetails.value[no]) {
              lampGroupDetails.value[no].status = status.lampStatus
            }
          })
        }
      })
    }
  } catch (e) {
    console.error('获取运行信息失败:', e)
    runInfo.value = {
      crossCtrlInfo: null,
      cycleInfo: null,
      stageInfo: null,
      crossSignalGroupStatus: null,
      signalGroupInfo: null
    }
  }
}

const formatLampStatus = (lampStatus) => {
  if (!lampStatus || typeof lampStatus !== 'string' || lampStatus.length !== 3) {
    return '无效灯态'
  }

  const statusMap = {
    '0': '无',
    '1': '灭', 
    '2': '亮',
    '3': '闪'
  }

  const [r, y, g] = lampStatus.split('')
  return `红:${statusMap[r]} 黄:${statusMap[y]} 绿:${statusMap[g]}`
}

const goToParameterCheck = async () => {
  try {
   

    // 跳转到参数检查页面
    router.push({
      name: 'ParameterCheck'
    })
  } catch (e) {
    console.error('跳转参数检查页面失败:', e)
    ElMessage.error('跳转失败')
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




</script> 