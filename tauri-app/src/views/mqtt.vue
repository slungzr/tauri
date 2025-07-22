<template>
  <div class="mqtt-container">
    <div class="connection-form">
      <el-form :model="mqttConfig" label-width="120px">
        <el-form-item label="MQTT服务器地址">
          <el-input v-model="mqttConfig.host" placeholder="例如：ws://localhost:9001"></el-input>
        </el-form-item>
        <el-form-item label="客户端ID">
          <el-input v-model="mqttConfig.clientId" placeholder="例如：mqtt_client_001"></el-input>
        </el-form-item>
        <el-form-item label="用户名">
          <el-input v-model="mqttConfig.username"></el-input>
        </el-form-item>
        <el-form-item label="密码">
          <el-input v-model="mqttConfig.password" type="password"></el-input>
        </el-form-item>
        <el-form-item label="Topic前缀">
          <el-input v-model="mqttConfig.topicPrefix" placeholder="v2x/wx/2003/"></el-input>
        </el-form-item>
        <el-form-item label="保持连接">
          <el-switch v-model="mqttConfig.keepalive" :active-value="60" :inactive-value="0"></el-switch>
        </el-form-item>
        <el-form-item label="清除会话">
          <el-switch v-model="mqttConfig.clean"></el-switch>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="connectMqtt" :loading="connecting">
            {{ isConnected ? '断开连接' : '连接' }}
          </el-button>
        </el-form-item>
      </el-form>
    </div>

    <div class="stats-panel" v-if="isConnected">
      <el-card>
        <template #header>
          <div class="card-header">
            <span>路口统计</span>
          </div>
        </template>
        <div class="stats-content">
          <p>路口总数：{{ intersectionCount }}</p>
          <p>已连接的路口：</p>
          <div class="intersection-list">
            <el-tag 
              v-for="intersection in intersectionList" 
              :key="intersection.id"
              class="intersection-tag"
              :type="intersection.active ? 'success' : 'info'"
            >
              {{ intersection.id }}
            </el-tag>
          </div>
        </div>
      </el-card>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed } from 'vue'
import mqtt from 'mqtt'
import { ElMessage } from 'element-plus'

const mqttConfig = reactive({
  host: 'ws://localhost:9001',
  clientId: `mqtt_client_${Math.random().toString(16).slice(2, 8)}`,
  username: '',
  password: '',
  topicPrefix: 'v2x/wx/2003/',
  keepalive: 60,
  clean: true
})

const client = ref(null)
const isConnected = ref(false)
const connecting = ref(false)
const intersectionList = ref([])
const lastMessageTime = ref({})

const intersectionCount = computed(() => intersectionList.value.length)

// 检查消息是否超时（30秒无消息视为离线）
const checkMessageTimeout = () => {
  const now = Date.now()
  intersectionList.value.forEach(intersection => {
    const lastTime = lastMessageTime.value[intersection.id] || 0
    intersection.active = (now - lastTime) < 120000
  })
}

// 定期检查消息超时
setInterval(checkMessageTimeout, 5000)

const connectMqtt = async () => {
  if (isConnected.value) {
    client.value.end()
    isConnected.value = false
    intersectionList.value = []
    lastMessageTime.value = {}
    return
  }

  connecting.value = true
  try {
    const connectUrl = mqttConfig.host
    const options = {
      clientId: mqttConfig.clientId,
      username: mqttConfig.username,
      password: mqttConfig.password,
      clean: mqttConfig.clean,
      keepalive: mqttConfig.keepalive,
      reconnectPeriod: 0,
      connectTimeout: 5000,
      protocol: 'ws',
      path: '/mqtt'
    }

    client.value = mqtt.connect(connectUrl, options)

    // 设置连接超时
    const connectTimeout = setTimeout(() => {
      if (!isConnected.value) {
        client.value.end()
        connecting.value = false
        ElMessage.error('连接超时，请检查服务器地址和网络')
      }
    }, 5000)

    client.value.on('connect', () => {
      clearTimeout(connectTimeout)
      isConnected.value = true
      connecting.value = false
      ElMessage.success('MQTT连接成功')
      
      // 订阅所有匹配前缀的topics
      client.value.subscribe(`${mqttConfig.topicPrefix}+`, (err) => {
        if (err) {
          ElMessage.error('订阅失败：' + err.message)
        }
      })
    })

    client.value.on('message', (topic, message) => {
      // 从topic中提取路口号
      const intersectionId = topic.replace(mqttConfig.topicPrefix, '')
      if (intersectionId) {
        // 更新最后消息时间
        lastMessageTime.value[intersectionId] = Date.now()
        
        // 如果是新的路口，添加到列表
        if (!intersectionList.value.find(i => i.id === intersectionId)) {
          intersectionList.value.push({
            id: intersectionId,
            active: true
          })
        }
      }
    })

    client.value.on('error', (err) => {
      clearTimeout(connectTimeout)
      ElMessage.error('MQTT错误：' + err.message)
      connecting.value = false
      isConnected.value = false
      client.value.end()
    })

    client.value.on('close', () => {
      clearTimeout(connectTimeout)
      isConnected.value = false
      connecting.value = false
      ElMessage.info('MQTT连接已关闭')
    })

  } catch (error) {
    ElMessage.error('连接失败：' + error.message)
    connecting.value = false
  }
}
</script>

<style scoped>
.mqtt-container {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.connection-form {
  margin-bottom: 20px;
}

.stats-panel {
  margin-top: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.stats-content {
  margin-top: 10px;
}

.intersection-list {
  margin-top: 10px;
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.intersection-tag {
  margin: 5px;
}
</style>
