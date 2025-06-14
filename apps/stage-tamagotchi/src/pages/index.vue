<script setup lang="ts">
import { WidgetStage } from '@proj-airi/stage-ui/components'
import { useMcpStore } from '@proj-airi/stage-ui/stores'
import { connectServer } from '@proj-airi/tauri-plugin-mcp'
import { invoke } from '@tauri-apps/api/core'
import { storeToRefs } from 'pinia'
import { computed, onMounted } from 'vue'

import { useWindowShortcuts } from '../composables/window-shortcuts'
import { useWindowControlStore } from '../stores/window-controls'
import { WindowControlMode } from '../types/window-controls'

const windowStore = useWindowControlStore()
useWindowShortcuts()

const mcpStore = useMcpStore()
const { connected, serverCmd, serverArgs } = storeToRefs(mcpStore)

const modeIndicatorClass = computed(() => {
  switch (windowStore.controlMode) {
    case WindowControlMode.MOVE:
      return 'cursor-move'
    case WindowControlMode.RESIZE:
      return 'cursor-se-resize'
    case WindowControlMode.DEBUG:
      return 'debug-mode'
    default:
      return ''
  }
})

function openSettings() {
  invoke('open_settings_window')
}

function openChat() {
  invoke('open_chat_window')
}

onMounted(async () => {
  if (connected.value)
    return
  if (!serverCmd.value || !serverArgs.value)
    return
  try {
    await connectServer(serverCmd.value, serverArgs.value.split(' '))
    connected.value = true
  }
  catch (error) {
    console.error(error)
  }
})
</script>

<template>
  <div
    :class="[modeIndicatorClass]"
    relative
    max-h="[100vh]"
    max-w="[100vw]"
    p="2"
    flex="~ col"
    z-2
    h-full
    overflow-hidden
  >
    <div relative h-full w-full items-end gap-2 class="view">
      <WidgetStage h-full w-full flex-1 mb="<md:18" />
      <div
        absolute bottom-4 left-4 flex gap-1 op-0 transition="opacity duration-250"
        class="interaction-area"
        :class="{ 'pointer-events-none': windowStore.isControlActive }"
      >
        <div
          border="solid 2 primary-100 "
          text="lg primary-400 hover:primary-600  placeholder:primary-400 placeholder:hover:primary-600"
          bg="primary-50 dark:primary-50" max-h="[10lh]" min-h="[1lh]"
          flex cursor-pointer items-center justify-center rounded-l-xl p-4 transition-colors
          @click="openChat"
        >
          <div i-solar:chat-line-bold-duotone />
        </div>
        <div
          border="solid 2 primary-100 "
          text="lg primary-400 hover:primary-600  placeholder:primary-400 placeholder:hover:primary-600"
          bg="primary-50 dark:primary-50" max-h="[10lh]" min-h="[1lh]"
          flex cursor-pointer items-center justify-center rounded-r-xl p-4 transition-colors
          @click="openSettings"
        >
          <div i-solar:settings-bold-duotone />
        </div>
      </div>
    </div>

    <!-- Debug Mode UI -->
    <div v-if="windowStore.controlMode === WindowControlMode.DEBUG" class="debug-controls">
      <!-- Add debug controls here -->
    </div>
  </div>
  <Transition
    enter-active-class="transition-opacity duration-250"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition-opacity duration-250"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div
      v-if="windowStore.controlMode === WindowControlMode.MOVE"
      data-tauri-drag-region
      class="drag-region absolute left-0 top-0 z-999 h-full w-full flex items-center justify-center overflow-hidden"
    >
      <div class="absolute h-32 w-full flex items-center justify-center b-2 b-pink bg-white">
        <div class="wall absolute top-0 h-8" />
        <div data-tauri-drag-region class="absolute left-0 top-0 h-full w-full flex animate-flash animate-duration-5s animate-count-infinite items-center justify-center text-1.5rem text-primary-300 font-bold">
          DRAG HERE TO MOVE
        </div>
        <div data-tauri-drag-region class="wall absolute bottom-0 h-8" />
      </div>
    </div>
  </Transition>
  <Transition
    enter-active-class="transition-opacity duration-250"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition-opacity duration-250"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div
      v-if="windowStore.controlMode === WindowControlMode.RESIZE"
      class="absolute left-0 top-0 z-999 h-full w-full"
    >
      <div h-full w-full animate-flash animate-duration-2.5s animate-count-infinite b-8 b-pink rounded-2xl />
    </div>
  </Transition>
</template>

<style scoped>
.view {
  &:hover {
    .interaction-area {
      opacity: 1;
      pointer-events: auto;
    }
  }
}

.drag-region {
  app-region: drag;
  cursor: grab !important;
}

@keyframes wall-move {
  0% {
    transform: translateX(calc(var(--wall-width) * -2));
  }
  100% {
    transform: translateX(calc(var(--wall-width) * 1));
  }
}

.wall {
  --at-apply: text-primary-200;

  --wall-width: 8px;
  animation: wall-move 1s linear infinite;
  background-image: repeating-linear-gradient(
    45deg,
    currentColor,
    currentColor var(--wall-width),
    #ff00 var(--wall-width),
    #ff00 calc(var(--wall-width) * 2)
  );
  width: calc(100% + 4 * var(--wall-width));
}
</style>

<route lang="yaml">
meta:
  layout: stage
</route>
