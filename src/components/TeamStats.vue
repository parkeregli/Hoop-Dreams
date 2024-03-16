<template>
  <card class="border-solid border-cyan-600">
    <template #title>
      <h1 class="title">Team Stats</h1>
    </template>
    <template class="flex flex-initial" #content>
      <DataView class="overflow-scroll min-h-0" :value="teamStats">
        <template #list="slotProps">
          <div class="flex flex-column">
            <div
              class="flex flex-wrap justify-content-between w-full"
              v-for="(item, index) in slotProps.items"
              :key="index"
            >
              <div class="stat">{{ item.stat }}</div>

              <div class="flex gap-7">
                <span class="home"> {{ item.home }} </span>

                {{ item.away }}
              </div>
            </div>
          </div>
        </template>
      </DataView>
    </template>
  </card>
</template>

<script setup lang="ts">
const props = defineProps({ teamStats: []});
</script>

<style scoped>
.title {
  text-align: center;
}
</style>
