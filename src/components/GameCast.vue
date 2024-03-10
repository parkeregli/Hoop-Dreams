<template>
  <Card class="card">
    <template #title>
      <h1 class="title">Game Cast</h1>
    </template>
    <template #content>
      <DataView :value="items">
        <template #list="slotProps">
          <div class="flex flex-column">
            <div v-for="(item, index) in slotProps.items" :key="index">
              <span class="timestamp"> {{ item.timestamp }} </span> -
              {{ item.name }}
            </div>
          </div>
        </template>
      </DataView>
    </template>
  </Card>
</template>

<script setup lang="ts">
const items = [
  { timestamp: "1:00", name: "Item 1" },
  { timestamp: "2:00", name: "Item 2" },
  { timestamp: "3:00", name: "Item 3" },
];
</script>

<style scoped>
.timestamp {
  border: 2px solid rgba(255, 230, 0, 0.767);
  padding: 2px 5px;
}
.title {
  text-align: center;
}
.card {
  width: 100%;
  height: 200px;
}
</style>
