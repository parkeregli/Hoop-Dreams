<template>
  <div class="flex justify-content-center">
    <Card class="homeCard">
      <template #title><h1>Hoop Dreams</h1></template>
      <template #content>
        <Menu :model="items">
          <template #item="{ item, props }">
            <router-link
              v-if="item.route"
              v-slot="{ href, navigate }"
              :to="item.route"
              custom
            >
              <a v-ripple :href="href" v-bind="props.action" @click="navigate">
                <span :class="item.icon" />
                <span class="ml-2">{{ item.label }}</span>
              </a>
            </router-link>
            <a
              v-else
              v-ripple
              :href="item.url"
              :target="item.target"
              v-bind="props.action"
            >
              <span :class="item.icon" />
              <span class="ml-2">{{ item.label }}</span>
            </a>
          </template>
        </Menu>
      </template>
    </Card>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

const items = ref([
  {
    label: "Coach Game",
    icon: "pi pi-palette",
    command: () => {
      router.push("/game");
    },
  },
  {
    label: "Career Mode",
    icon: "pi pi-link",
  },
  {
    label: "Options",
    icon: "pi pi-home",
  },

  {
    label: "Exit",
    icon: "pi pi-home",
  },
]);
</script>

<style>
.homeCard {
  width: 400px;
  height: 600px;
  align-items: center;
}

body {
  display: flex;
  align-items: center;
  background-color: steelblue;
}
</style>
