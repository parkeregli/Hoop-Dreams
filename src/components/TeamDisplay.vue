<template>
  <card class="homeAway border-solid border-cyan-600">
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
  const props = defineProps({ players: {type: Array, default:[]}, name: {type: String, default: ""}});
</script>

<style scoped>
.title {
  text-align: center;
}
</style>
