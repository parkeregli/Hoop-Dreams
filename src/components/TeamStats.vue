<template>
  <card>
    <template #title>
      <h1 class="title">Team Stats</h1>
    </template>
    <template #content>
      <DataView :value="teamStats">
        <template #list="slotProps">
          <div class="flex flex-column">
            <div v-for="(item, index) in slotProps.items" :key="index">
              <span class="stat"> {{ item.stat }} </span> -
              <span class="home"> {{ item.home }} </span> -

              {{ item.away }}
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

.card {
  width: 100%;
  height: 200px;
}
</style>
