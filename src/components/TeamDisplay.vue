<template>
  <card class="homeAway border-solid border-cyan-600">
    <template #title>
      <h1 class="title">{{ team_name }}</h1>
    </template>
    <template #content>
      <DataView :value="mappedPlayers">
        <template #list="slotProps">
          <div class="flex flex-column">
            <div v-for="(item, index) in slotProps.items" :key="index">
              <span class="position"> {{ item.position }} </span> -
              {{ item.first_name }} {{ item.last_name }}
            </div>
          </div>
        </template>
      </DataView>
    </template>
  </card>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted, computed } from "vue";
const props = defineProps({ team_name: { type: String, required: true }, players: { type: Array, required: true } });
const mappedPlayers = computed(() => {
  return props.players.map((player) => {
    return {
      position: player[0].position,
      first_name: player[0].first_name,
      last_name: player[0].last_name
    };
  });

});
onMounted(() => {


})
</script>

<style scoped>
.title {
  text-align: center;
}
</style>
