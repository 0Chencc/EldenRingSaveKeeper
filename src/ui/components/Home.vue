<script setup>
import {computed, onMounted, ref, watch} from 'vue';
import {invoke} from '@tauri-apps/api/tauri';
import {appWindow} from '@tauri-apps/api/window';
import {process} from "@tauri-apps/api";
const name = ref('');
const saveDirPath = ref('')

const selectedItem = ref('');
const items = ref([]);  // Tauri 会填充这个数组
const files = ref([]);  // 文件名列表
const isChecked = ref(true);
function customConfirm(message){
  return new Promise((resolve) => {
    const result = confirm(message)
    resolve(result)
  });
}
// const handleCheckChange = async() => {
//   isChecked.value = !isChecked;
// }
const handleManualSave = async() => {
  try {
    await invoke("manual_save")
    await getSaveZip();
  }catch (error){
    console.error()
  }

}
//button-group 功能点
const handleUnzip = async (i) => {
  const yesOrNot = ref(false)
  yesOrNot.value = await customConfirm("Are you sure you want to recover this save?")
  console.log(yesOrNot.value)
  if (yesOrNot.value) {
    await invoke('unzip_file', {path: files.value[i]})
    await customConfirm("have recovered this save")
  }
};

const deleteFile = async(i) => {
  const yesOrNot = ref(false)
  yesOrNot.value = await customConfirm("Are you sure you want to delete this file?")
  console.log(yesOrNot.value)
  if (yesOrNot.value) {
    await invoke('delete_save', {path: files.value[i]})
    await getSaveZip();
  }
}

const getInfo = (i) => {
  invoke('zip_info',{path:files.value[i]})
}

const getSaveZip = async() => {
  files.value = await invoke('find_save_archived')
}
const saveDirExists = async() =>{
  try {
     // 调用后端命令
    saveDirPath.value = await invoke('get_save_path'); // 更新响应式引用的值
  } catch (error) {
    console.error('Failed to fetch the save directory path:', error);
    saveDirPath.value = '获取失败'; // 可以设置一个错误消息
  }
}
const startDrag = () => {
  appWindow.startDragging();
};

const minimizeWindow = async () => {
  try {
    await appWindow.minimize();
  } catch (err) {
    console.error('Failed to minimize window:', err);
  }
};

const handleCheckChange = () =>{
  invoke('change_save_status',{status:isChecked})
}
const closeApp = async () => {
  try {
    await process.exit(); // Changed from process.exit to appWindow.close
  } catch (err) {
    console.error('Failed to close app:', err);
  }
};

// 提取数字序列
const uniqueIds = computed(() => {
  const ids = new Set();
  files.value.forEach(file => {
    const match = file.match(/(\d+)-\d{4}-\d{2}-\d{2} \d{2}_\d{2}_\d{2}_(manual|auto)\.zip$/);
    if (match) {
      ids.add(match[1]);
    }
  });
  return Array.from(ids);
});

// 选择的ID
const selectedId = ref('all');

// 根据选择的ID过滤文件名
const filteredFiles = computed(() => {
  if (selectedId.value === 'all') {
    return files.value.map(file => file.split('\\').pop().replace('.zip', ''));
  }
  return files.value.filter(file => file.includes(selectedId.value))
      .map(file => file.split('\\').pop().replace('.zip', ''));
});

// 当前选中项的索引
const selectedFile = ref(null);
// 点击列表项时调用的方法
const selectItem = (file) => {
  // 如果已选中的文件再次被点击，取消选中，否则设置为选中
  selectedFile.value = selectedFile.value === file ? null : file;
};
// 监听选择的改变，更新文件列表
watch(selectedId, (newVal) => {
  // 更新文件列表逻辑，如果需要
});
const openNewWindow = async(filePath) => {
  const newWindow = await appWindow.create({
    url: `/detail.html?filePath=${encodeURIComponent(filePath)}`,
    title: 'File Info',
    width: 700,
    height: 600,
    x: window.screenX + window.outerWidth,
    y: window.screenY
  });
};


onMounted(() => {
  saveDirExists();
  getSaveZip();
});
</script>

<template>
  <div class="window-container">
    <div class="drag-area" @mousedown="startDrag"></div>
    <div class="window-controls">
      <button @click="minimizeWindow" @mousedown.stop>
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-dash" viewBox="0 0 16 16">
          <path d="M4 8a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7A.5.5 0 0 1 4 8"/>
        </svg>
      </button>
<!--      <button @click="setting" @mousedown.stop>-->
<!--        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-gear" viewBox="0 0 16 16">-->
<!--          <path d="M8 4.754a3.246 3.246 0 1 0 0 6.492 3.246 3.246 0 0 0 0-6.492M5.754 8a2.246 2.246 0 1 1 4.492 0 2.246 2.246 0 0 1-4.492 0"/>-->
<!--          <path d="M9.796 1.343c-.527-1.79-3.065-1.79-3.592 0l-.094.319a.873.873 0 0 1-1.255.52l-.292-.16c-1.64-.892-3.433.902-2.54 2.541l.159.292a.873.873 0 0 1-.52 1.255l-.319.094c-1.79.527-1.79 3.065 0 3.592l.319.094a.873.873 0 0 1 .52 1.255l-.16.292c-.892 1.64.901 3.434 2.541 2.54l.292-.159a.873.873 0 0 1 1.255.52l.094.319c.527 1.79 3.065 1.79 3.592 0l.094-.319a.873.873 0 0 1 1.255-.52l.292.16c1.64.893 3.434-.902 2.54-2.541l-.159-.292a.873.873 0 0 1 .52-1.255l.319-.094c1.79-.527 1.79-3.065 0-3.592l-.319-.094a.873.873 0 0 1-.52-1.255l.16-.292c.893-1.64-.902-3.433-2.541-2.54l-.292.159a.873.873 0 0 1-1.255-.52zm-2.633.283c.246-.835 1.428-.835 1.674 0l.094.319a1.873 1.873 0 0 0 2.693 1.115l.291-.16c.764-.415 1.6.42 1.184 1.185l-.159.292a1.873 1.873 0 0 0 1.116 2.692l.318.094c.835.246.835 1.428 0 1.674l-.319.094a1.873 1.873 0 0 0-1.115 2.693l.16.291c.415.764-.42 1.6-1.185 1.184l-.291-.159a1.873 1.873 0 0 0-2.693 1.116l-.094.318c-.246.835-1.428.835-1.674 0l-.094-.319a1.873 1.873 0 0 0-2.692-1.115l-.292.16c-.764.415-1.6-.42-1.184-1.185l.159-.291A1.873 1.873 0 0 0 1.945 8.93l-.319-.094c-.835-.246-.835-1.428 0-1.674l.319-.094A1.873 1.873 0 0 0 3.06 4.377l-.16-.292c-.415-.764.42-1.6 1.185-1.184l.292.159a1.873 1.873 0 0 0 2.692-1.115z"/>-->
<!--        </svg>-->
<!--      </button>-->
      <button @click="closeApp" @mousedown.stop>
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-x" viewBox="0 0 16 16">
          <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708"/>
        </svg>
      </button>
    </div>
    <div class="content">
      <img src="../assets/icon.png" alt="Logo" class="logo">
      <h1>Elden Ring Save Keeper</h1>
      <div class="label-button-container">
        <label for="savePathButton">Save Path:</label>
        <button id="savePathButton">{{ saveDirPath }}</button>
      </div>
      <div class="select-and-list">
        <div class="select-button-container">
          <select v-model="selectedId">
            <option value="all">All</option>
            <option v-for="id in uniqueIds" :key="id" :value="id">{{ id }}</option>
          </select>
          <button @click="getSaveZip">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-repeat" viewBox="0 0 16 16">
              <path d="M11.534 7h3.932a.25.25 0 0 1 .192.41l-1.966 2.36a.25.25 0 0 1-.384 0l-1.966-2.36a.25.25 0 0 1 .192-.41m-11 2h3.932a.25.25 0 0 0 .192-.41L2.692 6.23a.25.25 0 0 0-.384 0L.342 8.59A.25.25 0 0 0 .534 9"/>
              <path fill-rule="evenodd" d="M8 3c-1.552 0-2.94.707-3.857 1.818a.5.5 0 1 1-.771-.636A6.002 6.002 0 0 1 13.917 7H12.9A5 5 0 0 0 8 3M3.1 9a5.002 5.002 0 0 0 8.757 2.182.5.5 0 1 1 .771.636A6.002 6.002 0 0 1 2.083 9z"/>
            </svg>
          </button>
        </div>
        <ul>
          <li v-for="(file,index) in filteredFiles" :key="index"
              :class="{ 'selected': selectedFile === file }"
              @click="selectItem(file)">
            {{ file }}
            <div class="button-group">
              <button @click.stop.prevent="handleUnzip(index)">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-play" viewBox="0 0 16 16">
                  <path d="M10.804 8 5 4.633v6.734zm.792-.696a.802.802 0 0 1 0 1.392l-6.363 3.692C4.713 12.69 4 12.345 4 11.692V4.308c0-.653.713-.998 1.233-.696z"/>
                </svg>
              </button>
              <button @click.stop.prevent="deleteFile(index)">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                  <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0z"/>
                  <path d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4zM2.5 3h11V2h-11z"/>
                </svg>
              </button>
              <button @click.stop="getInfo(index)">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-info-circle" viewBox="0 0 16 16">
                  <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14m0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16"/>
                  <path d="m8.93 6.588-2.29.287-.082.38.45.083c.294.07.352.176.288.469l-.738 3.468c-.194.897.105 1.319.808 1.319.545 0 1.178-.252 1.465-.598l.088-.416c-.2.176-.492.246-.686.246-.275 0-.375-.193-.304-.533zM9 4.5a1 1 0 1 1-2 0 1 1 0 0 1 2 0"/>
                </svg>
              </button>
            </div>
          </li>
        </ul>
        <div class="checkbox-container">
          <input type="checkbox" id="checkboxInput" v-model="isChecked" @change="handleCheckChange" @click="">
          <label for="checkboxInput" class="toggleSwitch"></label>
          <label class="auto-save-label">Automatic Archiving</label>
          <button class="manual-save-button" @click="handleManualSave">Manual Save</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.logo {
  position: absolute;  /* 绝对定位，相对于最近的已定位父元素 */
  top: 27px;              /* 距离页面顶部 0 像素 */
  left: 65px;             /* 距离页面左侧 0 像素 */
  height: 35px;
}
.window-container {
  position: relative;
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
}
.drag-area {
  flex: 0 0 10%;  /* 高度设置为容器高度的10% */
  width: 100%;     /* 宽度设置为100% */
}
.content {
  flex: 1 1 90%;  /* 余下的90%高度分配给内容区 */
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.label-button-container {
  width: 525px;  /* 与选择框相同的宽度 */
  display: flex;
  justify-content: space-between;  /* 元素之间均匀分布 */
  margin-top: 20px;  /* 在上方添加20px的间距 */
  margin-bottom: 10px;
}

.label-button-container label {
  flex: 1;  /* label 占用可用空间的一部分 */
  font-size: 1.2rem;
  font-weight: bold;
}

.label-button-container button {
  flex: 3;  /* button 占用可用空间的更大部分 */
  color: white;
  background-color: #222;
  font-weight: 525;
  border-radius: 0.5rem;
  font-size: 0.8rem;
  line-height: 1.5rem;
  padding-left: 1rem;
  padding-right: 1rem;
  padding-top: 0.3rem;
  padding-bottom: 0.3rem;
  cursor: pointer;
  text-align: center;
  margin-right: 0.5rem;
  display: inline-flex;
  align-items: center;
  border: none;
}
.label-button-container button:hover{
  background-color: #333;
}
.buttons button {
  flex: 1;
  margin: 0 100px; /* 两个按钮之间保持200px的距离 */
}

.select-and-list select{
  width: 525px;  /* 设置宽度为400px */
}
.select-and-list ul {
  width: 525px;  /* 设置宽度为400px */
  min-height: 50px;  /* 设置一个最小高度 */
  background-color: #f0f0f0;  /* 背景色可以帮助视觉上辨认元素位置 */
  padding: 0;  /* 添加一些内边距 */
}
.select-and-list {
  width: 525px;  /* 设置宽度为525px */
  display: flex;
  flex-direction: column;
  align-items: flex-start;  /* 子元素靠左对齐 */
  margin-bottom: 10px;  /* 在下方添加20px的间距 */
}

.select-button-container {
  display: flex;
  justify-content: space-between;  /* 选择框和按钮分布在容器两侧 */
  width: 100%;  /* 设置容器宽度为100% */
  margin-bottom: 10px;  /* 添加间距 */
}

ul {
  list-style-type: none;
  padding: 0;
  height: 300px;
  overflow-y: auto;
}

ul li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  cursor: pointer;
  background-color: #f9f9f9;
}

ul li:nth-child(even) {
  background-color: #ffffff;
}

ul li:hover {
  background-color: #e8e8e8;
}

ul li.selected {
  background-color: #ccc;
}

.button-group {
  display: flex;  /* 确保按钮在一行显示 */
}

.button-group button {
  background: none;
  border: none;
  padding: 5px;
  cursor: pointer;
}

.button-group button svg {
  width: 20px;  /* 调整SVG图标的大小 */
  height: 20px;
  fill: currentColor;
}

.button-group button:hover {
  background-color: rgba(0, 0, 0, 0.1);  /* 悬停时轻微的背景变化 */
}

.button-group button:focus {
  outline: none;  /* 移除焦点轮廓 */
}

/* To hide the checkbox */
#checkboxInput {
  display: none;
}

.toggleSwitch {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  width: 50px;
  height: 30px;
  background-color: rgb(82, 82, 82);
  border-radius: 20px;
  cursor: pointer;
  transition-duration: .2s;
}

.toggleSwitch::after {
  content: "";
  position: absolute;
  height: 10px;
  width: 10px;
  left: 5px;
  background-color: transparent;
  border-radius: 50%;
  transition-duration: .2s;
  box-shadow: 5px 2px 7px rgba(8, 8, 8, 0.26);
  border: 5px solid white;
}

#checkboxInput:checked+.toggleSwitch::after {
  transform: translateX(100%);
  transition-duration: .2s;
  background-color: white;
}
/* Switch background change */
#checkboxInput:checked+.toggleSwitch {
  background-color: rgb(148, 118, 255);
  transition-duration: .2s;
}
.checkbox-container {
  display: flex;
  align-items: center;  /* 确保所有子项垂直居中 */
  justify-content: space-between;  /* 子项间隔均匀，确保按钮靠右 */
  width: 525px;  /* 与 list 宽度相同 */
}
.manual-save-button {
  margin-left: auto;  /* 将按钮推向右边 */
  padding: 5px 10px;  /* 按钮的内边距，可调整 */
  cursor: pointer;  /* 更改鼠标为指针 */
  color: white;
  background-color: #222;
  font-weight: 525;
  border-radius: 0.5rem;
  font-size: 0.9rem;
  line-height: 2rem;
  text-align: center;
  margin-right: 0.5rem;
  display: inline-flex;
  align-items: center;
  border: none;
}

.manual-save-button:hover {
  background-color: #333;
}

.auto-save-label {
  margin-left: 10px;  /* 保持左边距 */
  vertical-align: middle;  /* 垂直居中 */
  font-size: 1.2rem;  /* 字体大小 */
  color: #333;  /* 改为深灰色，增强可读性 */
  display: inline-block;  /* 确保以块状元素显示 */
}
</style>
