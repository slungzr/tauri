<template>
  <div class="login-container">
    <el-card class="login-card">
      <template #header>
        <div class="card-header">
          <h2>Redis 连接</h2>
        </div>
      </template>
      
      <el-form :model="form" :rules="rules" ref="formRef" label-width="80px">
        <el-form-item label="标准" prop="standard">
          <el-select v-model="form.standard" placeholder="请选择标准" style="width: 100%">
            <el-option label="GA/T 1049" value="1049" />
            <el-option label="GB" value="GB" />
          </el-select>
        </el-form-item>

        <el-form-item label="地址" prop="host">
          <el-input v-model="form.host" placeholder="localhost">
            <template #prepend>Host</template>
          </el-input>
        </el-form-item>
        
        <el-form-item label="端口" prop="port">
          <el-input v-model.number="form.port" placeholder="6379">
            <template #prepend>Port</template>
          </el-input>
        </el-form-item>
        
        <el-form-item label="密码" prop="password">
          <el-input v-model="form.password" type="password" placeholder="可选">
            <template #prepend>Password</template>
          </el-input>
        </el-form-item>

        <el-form-item>
          <el-button type="primary" @click="handleConnect" :loading="loading">
            连接
          </el-button>
          <el-button type="success" @click="goToMQTT">
            MQTT监控
          </el-button>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { invoke } from '@tauri-apps/api/tauri'

const router = useRouter()
const formRef = ref(null)
const loading = ref(false)

const form = reactive({
  host: 'localhost',
  port: 6379,
  password: '',
  standard: '1049'
})

const rules = {
  standard: [
    { required: true, message: '请选择标准', trigger: 'change' }
  ],
  host: [
    { required: true, message: '请输入Redis服务器地址', trigger: 'blur' }
  ],
  port: [
    { required: true, message: '请输入端口号', trigger: 'blur' },
    { type: 'number', message: '端口号必须为数字', trigger: 'blur' }
  ]
}

const goToMQTT = () => {
  router.push('/mqtt')
}

const handleConnect = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (valid) {
      loading.value = true
      try {
        // 调用 Tauri 命令连接 Redis
        const result = await invoke('connect_redis', {
          config: {
            host: form.host,
            port: form.port,
            password: form.password || undefined
          }
        })
        
        if (result.success) {
          localStorage.setItem('redis-connected', 'true')
          localStorage.setItem('redis-config', JSON.stringify(form))
          ElMessage.success('连接成功')
          // 根据标准选择跳转到不同页面
          if (form.standard === '1049') {
            router.push('/browser')
          } else {
            router.push('/gb')
          }
        } else {
          ElMessage.error(result.error || '连接失败')
        }
      } catch (error) {
        ElMessage.error(error.toString())
      } finally {
        loading.value = false
      }
    }
  })
}
</script>

<style scoped>
.login-container {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #f5f7fa;
}

.login-card {
  width: 400px;
}

.card-header {
  text-align: center;
}

.card-header h2 {
  margin: 0;
  color: #409EFF;
}
</style> 