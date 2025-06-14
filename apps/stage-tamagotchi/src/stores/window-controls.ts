import { useLocalStorage } from '@vueuse/core'
import { defineStore } from 'pinia'
import { ref } from 'vue'

import { WindowControlMode } from '../types/window-controls'

export const useWindowControlStore = defineStore('windowControl', () => {
  const controlMode = ref<WindowControlMode>(WindowControlMode.DEFAULT)
  const isControlActive = ref(false)
  const size = useLocalStorage('window/control/size', { width: 300 * 1.5, height: 400 * 1.5 })
  const position = useLocalStorage('window/control/position', { x: 0, y: 0 })

  function setMode(mode: WindowControlMode) {
    controlMode.value = mode

    if (mode === WindowControlMode.RESIZE) {
      // return
    }
  }

  function toggleControl() {
    isControlActive.value = !isControlActive.value
    if (!isControlActive.value) {
      controlMode.value = WindowControlMode.DEFAULT
    }
  }

  return {
    controlMode,
    isControlActive,
    size,
    position,
    setMode,
    toggleControl,
  }
})
