<template>
  <div class="page-container">
    <TopMenu />
    <div class="parameter-check">
      <h2>Redis参数检查</h2>
      
      <div class="scan-section">
        <div class="scan-input">
          <el-input
            v-model="keyPrefix"
            placeholder="请输入Redis key前缀"
            style="width: 300px; margin-right: 10px;"
            :disabled="scanning"
          />
          <el-input
            v-model="validationRules"
            type="textarea"
            placeholder="请输入校验规则（JSON格式）"
            :rows="4"
            style="width: 400px; margin-right: 10px;"
            :disabled="scanning"
          />
          <el-button type="primary" @click="scanRedisKeys" :loading="scanning">
            开始检查
          </el-button>
        </div>
        
        <div class="rules-guide">
          <h3>校验规则说明</h3>
          <div class="rules-examples">
           

            <div class="rule-example">
              <h4>1. 路口参数校验示例</h4>
              <pre>{
  "crossID": {
    "type": "string",
    "required": true
  },
  "crossName": {
    "type": "string",
    "required": true
  },
  "feature": {
    "type": "number",
    "required": true,
    "enum": [10, 12, 23, 24, 35, 36, 39, 40, 50, 51, 52, 61, 90]
  },
  "grade": {
    "type": "string",
    "required": true,
    "enum": ["11", "12", "13", "21", "22", "31", "99"]
  },
  "detNoList": {
    "type": "object",
    "required": true,
    "properties": {
      "detNo": {
        "type": "array",
        "required": true,
        "items": {
          "type": "number"
        }
      }
    }
  },
  "laneNoList": {
    "type": "object",
    "required": true,
    "properties": {
      "laneNo": {
        "type": "array",
        "required": true,
        "minLength": 1,
        "items": {
          "type": "number"
        }
      }
    }
  },
  "pedestrianNoList": {
    "type": "object",
    "required": true,
    "properties": {
      "pedestrianNo": {
        "type": "array",
        "required": true,
        "items": {
          "type": "number"
        }
      }
    }
  },
  "lampGroupNoList": {
    "type": "object",
    "required": true,
    "properties": {
      "lampGroupNo": {
        "type": "array",
        "required": true,
        "minLength": 1,
        "items": {
          "type": "number"
        }
      }
    }
  },
  "signalGroupNoList": {
    "type": "object",
    "required": true,
    "properties": {
      "signalGroupNo": {
        "type": "array",
        "required": true,
        "minLength": 1,
        "items": {
          "type": "number"
        }
      }
    }
  },
  "greenConflictMatrix": {
    "type": "string",
    "required": true
  },
  "stageNoList": {
    "type": "object",
    "required": true,
    "properties": {
      "stageNo": {
        "type": "array",
        "required": true,
        "minLength": 1,
        "items": {
          "type": "number"
        }
      }
    }
  },
  "planNoList": {
    "type": "object",
    "required": true,
    "properties": {
      "planNo": {
        "type": "array",
        "required": true,
        "minLength": 1,
        "items": {
          "type": "number"
        }
      }
    }
  },
  "dayPlanNoList": {
    "type": "object",
    "required": true,
    "properties": {
      "dayPlanNo": {
        "type": "array",
        "required": true,
        "minLength": 1,
        "items": {
          "type": "number"
        }
      }
    }
  },
  "scheduleNoList": {
    "type": "object",
    "required": true,
    "properties": {
      "scheduleNo": {
        "type": "array",
        "required": true,
        "minLength": 1,
        "items": {
          "type": "number"
        }
      }
    }
  },
  "longitude": {
    "type": "number",
    "required": true,
    "min": -180,
    "max": 180
  },
  "latitude": {
    "type": "number",
    "required": true,
    "min": -90,
    "max": 90
  },
  "altitude": {
    "type": "number",
    "required": false
  }
}</pre>
            </div>
          </div>
        </div>
        
        <div v-if="checkResults.length > 0" class="check-results">
          <h3>异常结果：</h3>
          <el-table :data="invalidResults" style="width: 100%">
            <el-table-column prop="key" label="键名" width="300" />
            <el-table-column prop="issues" label="问题描述" min-width="400">
              <template #default="scope">
                <div class="check-result">
                  <el-tag 
                    v-for="(issue, index) in scope.row.issues" 
                    :key="index"
                    type="danger"
                    class="mx-1"
                  >
                    {{ issue }}
                  </el-tag>
                </div>
              </template>
            </el-table-column>
            <el-table-column type="expand">
              <template #default="props">
                <div class="expanded-content">
                  <pre>{{ JSON.stringify(props.row.value, null, 2) }}</pre>
                </div>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </div>
      </div>
    </div>
  </template>
  
  <script>
import { invoke } from '@tauri-apps/api/tauri'
import TopMenu from '../components/TopMenu.vue'

  export default {
    name: 'ParameterCheck',
  components: {
    TopMenu
  },
    data() {
      return {
      keyPrefix: '',
      validationRules: '',
      scanning: false,
      checkResults: [],
      parsedRules: null,
      rulesExampleVisible: false,
      rulesExample: {
        title: '校验规则示例',
        content: `{
  // 基础类型检查
  "name": {
    "type": "string",     // 类型检查：string/number/array
    "required": true,     // 必填检查
    "minLength": 1,       // 最小长度
    "maxLength": 50       // 最大长度
  },
  
  // 数值范围检查
  "age": {
    "type": "number",
    "required": true,
    "min": 0,            // 最小值
    "max": 150           // 最大值
  },
  
  // 数组检查
  "tags": {
    "type": "array",
    "required": true,
    "minLength": 1,      // 数组最小长度
    "maxLength": 10      // 数组最大长度
  },
  
  // 嵌套对象检查
  "address": {
    "type": "object",
    "required": true,
    "properties": {      // 嵌套属性检查
      "city": {
        "type": "string",
        "required": true
      },
      "street": {
        "type": "string",
        "required": true
      }
    }
  },
  
  // 枚举值检查
  "status": {
    "type": "string",
    "required": true,
    "enum": ["active", "inactive", "pending"]  // 枚举值检查
  }
}`
      }
    }
  },
  methods: {
    async scanRedisKeys() {
      if (!this.keyPrefix) {
        this.$message.warning('请输入Redis key前缀')
        return
      }

      try {
        this.parsedRules = JSON.parse(this.validationRules)
      } catch (e) {
        this.$message.error('校验规则JSON格式错误')
        return
      }
      
      this.scanning = true
      this.checkResults = []
      console.log( this.keyPrefix)
          
          try {
        // 1. 搜索匹配的keys
        const searchResponse = await invoke('search_keys', {
          pattern: this.keyPrefix + '*'
              })
              
        if (searchResponse.success && searchResponse.data) {
          const keys = searchResponse.data

          // 2. 批量获取值
          const batchSize = 100
          for (let i = 0; i < keys.length; i += batchSize) {
            const batchKeys = keys.slice(i, i + batchSize)
            console.log(batchKeys)
            const batchData = await invoke('get_keys_data', { keys: batchKeys })

                          if (batchData.success && batchData.data) {
                            for (const item of batchData.data) {
                              if (item.success && item.data) {
                                try {
                    const value = item.data.value
                    if (typeof value === 'string') {
                      const parsedValue = JSON.parse(value)
                      const validationResult = this.validateValue(parsedValue)
                      
                      this.checkResults.push({
                        key: item.key,
                        value: parsedValue,
                        status: validationResult.status,
                        issues: validationResult.issues
                                    })
                                  }
                                } catch (e) {
                    this.checkResults.push({
                      key: item.key,
                      value: null,
                      status: 'invalid',
                      issues: ['JSON解析失败']
                    })
                    }
                  }
                }
              }
          }
        }
      } catch (error) {
        this.$message.error('检查失败: ' + error)
      } finally {
        this.scanning = false
      }
    },

    validateValue(value) {
      const issues = []
      
      // 遍历规则进行验证
      for (const [path, rules] of Object.entries(this.parsedRules)) {
        const fieldValue = this.getNestedValue(value, path)
        
        // 必填检查
        if (rules.required && (fieldValue === undefined || fieldValue === null || fieldValue === '')) {
          issues.push(`${path} 是必填项`)
        }
        
        // 类型检查
        if (fieldValue !== undefined && fieldValue !== null) {
          // 对象类型检查
          if (rules.type === 'object' && rules.properties) {
            for (const [propName, propRules] of Object.entries(rules.properties)) {
              const propValue = fieldValue[propName]
              const propPath = `${path}.${propName}`
              
              // 必填检查
              if (propRules.required && (propValue === undefined || propValue === null || propValue === '')) {
                issues.push(`${propPath} 是必填项`)
              }
              
              // 类型检查
              if (propValue !== undefined && propValue !== null) {
                // 数组类型检查
                if (propRules.type === 'array') {
                  if (!Array.isArray(propValue)) {
                    issues.push(`${propPath} 类型错误，期望 array，实际 ${typeof propValue}`)
                  } else {
                    // 数组长度检查
                    if (propRules.minLength !== undefined && propValue.length < propRules.minLength) {
                      issues.push(`${propPath} 数组长度小于最小值 ${propRules.minLength}`)
                    }
                    if (propRules.maxLength !== undefined && propValue.length > propRules.maxLength) {
                      issues.push(`${propPath} 数组长度大于最大值 ${propRules.maxLength}`)
                    }
                    
                    // 检查数组元素类型
                    if (propRules.items && propRules.items.type) {
                      propValue.forEach((item, index) => {
                        if (typeof item !== propRules.items.type) {
                          issues.push(`${propPath}[${index}] 类型错误，期望 ${propRules.items.type}，实际 ${typeof item}`)
                        }
                      })
                    }
                  }
                } else if (propRules.type && typeof propValue !== propRules.type) {
                  issues.push(`${propPath} 类型错误，期望 ${propRules.type}，实际 ${typeof propValue}`)
                }
              }
            }
          } else {
            // 非对象类型的检查
            if (rules.type === 'array') {
              if (!Array.isArray(fieldValue)) {
                issues.push(`${path} 类型错误，期望 array，实际 ${typeof fieldValue}`)
              } else {
                // 数组长度检查
                if (rules.minLength !== undefined && fieldValue.length < rules.minLength) {
                  issues.push(`${path} 数组长度小于最小值 ${rules.minLength}`)
                }
                if (rules.maxLength !== undefined && fieldValue.length > rules.maxLength) {
                  issues.push(`${path} 数组长度大于最大值 ${rules.maxLength}`)
                }
              }
            } else if (rules.type && typeof fieldValue !== rules.type) {
              issues.push(`${path} 类型错误，期望 ${rules.type}，实际 ${typeof fieldValue}`)
            }
            
            // 数值范围检查
            if (rules.type === 'number') {
              if (rules.min !== undefined && fieldValue < rules.min) {
                issues.push(`${path} 小于最小值 ${rules.min}`)
              }
              if (rules.max !== undefined && fieldValue > rules.max) {
                issues.push(`${path} 大于最大值 ${rules.max}`)
              }
            }
            
            // 字符串长度检查
            if (rules.type === 'string') {
              if (rules.minLength !== undefined && fieldValue.length < rules.minLength) {
                issues.push(`${path} 长度小于最小值 ${rules.minLength}`)
              }
              if (rules.maxLength !== undefined && fieldValue.length > rules.maxLength) {
                issues.push(`${path} 长度大于最大值 ${rules.maxLength}`)
              }
            }
          }
        }
      }
      
      return {
        status: issues.length === 0 ? 'valid' : 'invalid',
        issues: issues
      }
    },

    getNestedValue(obj, path) {
      return path.split('.').reduce((current, key) => {
        return current && current[key] !== undefined ? current[key] : undefined
      }, obj)
      }
    },

    computed: {
      invalidResults() {
        return this.checkResults.filter(result => result.status === 'invalid')
      }
    }
  }
  </script>
  
  <style scoped>
.page-container {
  min-height: 100vh;
  background-color: #f5f7fa;
}

  .parameter-check {
  padding: 80px 20px 20px;
}

.scan-section {
  max-width: 1600px;
  margin: 20px auto;
}

.scan-input {
  display: flex;
  align-items: flex-start;
  margin-bottom: 20px;
  gap: 10px;
}

.check-results {
  margin-top: 20px;
}

.mx-1 {
  margin: 0 4px;
  }
  
  .check-result {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.check-result .el-tag {
  margin: 2px;
  }
  
  h2 {
    text-align: center;
    color: #409EFF;
  }
  
  h3 {
    margin-bottom: 15px;
    color: #606266;
}

.expanded-content {
  padding: 20px;
  background-color: #f8f9fa;
  border-radius: 4px;
}

.expanded-content pre {
  margin: 0;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.rules-guide {
  margin: 20px 0;
  padding: 20px;
  background-color: #f8f9fa;
  border-radius: 8px;
}

.rules-guide h3 {
  margin-bottom: 20px;
  color: #409EFF;
  font-size: 18px;
}

.rules-examples {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 20px;
}

.rule-example {
  background-color: white;
  padding: 15px;
  border-radius: 6px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.rule-example h4 {
  margin: 0 0 10px 0;
  color: #606266;
  font-size: 16px;
}

.rule-example pre {
  margin: 0;
  padding: 10px;
  background-color: #f8f9fa;
  border-radius: 4px;
  font-family: Consolas, Monaco, 'Andale Mono', monospace;
  font-size: 13px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-wrap: break-word;
  color: #333;
}

.rule-example pre .comment {
  color: #999;
}
  </style>