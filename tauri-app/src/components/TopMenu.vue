<template>
  <div class="top-menu">
    <el-menu
      :default-active="activeIndex"
      class="el-menu-demo"
      mode="horizontal"
      @select="handleSelect"
      background-color="#545c64"
      text-color="#fff"
      active-text-color="#ffd04b">
      
      <el-menu-item index="1">
        <el-icon><Monitor /></el-icon>
        参数检查
      </el-menu-item>
      
      <el-menu-item index="2">
        <el-icon><Connection /></el-icon>
        浏览路口
      </el-menu-item>
      
      
      
      <div class="flex-grow" />
      
      <el-menu-item index="4">
        <el-icon><User /></el-icon>
        {{ username }}
      </el-menu-item>
    </el-menu>
  </div>
</template>

<script>
import { Monitor, Connection, Setting, User } from '@element-plus/icons-vue'

export default {
  name: 'TopMenu',
  components: {
    Monitor,
    Connection,
    Setting,
    User
  },
  data() {
    return {
      activeIndex: '1',
      username: '管理员'
    }
  },
  methods: {
    handleSelect(key) {
      switch(key) {
        case '1':
          this.$router.push('/parameter-check')
          break
        case '2':
          this.$router.push('/browser')
          break
        case '3':
          this.$router.push('/settings')
          break
        case '4':
          // 处理用户菜单点击
          break
      }
    }
  }
}
</script>

<style scoped>
.top-menu {
  width: 100%;
  position: fixed;
  top: 0;
  left: 0;
  z-index: 1000;
}

.el-menu-demo {
  display: flex;
  padding: 0 20px;
}

.flex-grow {
  flex-grow: 1;
}

:deep(.el-menu-item) {
  display: flex;
  align-items: center;
  gap: 4px;
}

:deep(.el-menu--horizontal) {
  border-bottom: none;
}

:deep(.el-menu-item.is-active) {
  border-bottom: 2px solid #ffd04b;
}
</style> 