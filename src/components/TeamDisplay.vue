<template>
  <card class="homeAway">
    <template #title>
      <h1 class="title">{{ name }}</h1>
    </template>
    <template #content>
      <DataView :value="players">
        <template #list="slotProps">
          <div class="flex flex-column">
            <div v-for="(item, index) in slotProps.items" :key="index">
              <span class="position"> {{ item.position }} </span> -
              {{ item.name }}
            </div>
          </div>
        </template>
      </DataView>
    </template>
  </card>
</template>

<script setup lang="ts">
const props = defineProps({ players: [], name: String });
</script>

<style scoped>
.title {
  text-align: center;
}
</style>
