<template>
  <div class="notification-container" @mousedown="startDrag">
    <div class="tips">The archive has been saved automatically</div>
    <p @mousedown.stop>{{savePath}}</p>
    <div class="window-controls">
      <button @click="closeWindow" @mousedown.stop>
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-x" viewBox="0 0 16 16">
          <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window';
import { invoke} from "@tauri-apps/api/tauri";
import {onMounted, ref} from "vue";

const savePath = ref('Saving...')
const closeWindow = async () => {
  try {
    await appWindow.close();
  } catch (err) {
    console.error('Failed to close window:', err);
  }
};

const startDrag = () => {
  appWindow.startDragging();
};

const autoSave = async() =>{
  savePath.value = await invoke('auto_save')
}
onMounted(autoSave)
</script>

<style scoped>
.notification-container {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-align: center;
  background-color: white;
  padding: 20px;
}
.tips {
  font-size: 1.2rem;
}
.notification-container p {
  font-size: 1rem;
}
</style>
