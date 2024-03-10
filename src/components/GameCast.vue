<template>
  <Card>
    <template #title>
      <h1>Game Cast</h1>
    </template>
    <template #content>
      <OrderList v-model="items" listStyle="height:auto">
        <template #item="slotProps">
          {{ slotProps.item.timestamp }} - 
          {{ slotProps.item.name }} 
        </template>
      </OrderList>
    </template>
  </Card>
</template>

<script setup lang="ts">
  const items = [
    { timestamp: '1:00', name: 'Item 1' },
    { timestamp: '2:00', name: 'Item 2' },
    { timestamp: '3:00', name: 'Item 3' },
  ]
</script>

<style scoped>
  
</style>
