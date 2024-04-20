<template>
  <div class="row align-items-center justify-content-center">
    <Menu :model="items" class="text-3xl p-6" :pt:label:class="'mb-4'">
      <template #start>
        <h1 class="text-7xl">
          Hoop Dreams
        </h1>
      </template>
      <template #item="{ item, props }">
        <router-link v-if="item.route" v-slot="{ href, navigate }" :to="item.route" custom>
          <a v-ripple :href="href" v-bind="props.action" @click="navigate">
            <span :class="item.icon" />
            <span class="ml-2">{{ item.label }}</span>
          </a>
        </router-link>
        <a v-else v-ripple :href="item.url" :target="item.target" v-bind="props.action">
          <span :class="item.icon" />
          <span class="ml-2">{{ item.label }}</span>
        </a>
      </template>
    </Menu>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

const items = ref([
  {
    label: "Coach Game",
    icon: "pi pi-caret-right",
    command: () => {
      router.push("/game");
    },
  },
  {
    label: "Career Mode",
    icon: "pi pi-caret-right",
  },
  {
    label: "Options",
    icon: "pi pi-caret-right",
  },

  {
    label: "Exit",
    icon: "pi pi-sign-out",
  },
]);
</script>

<style scoped>
.menuCard {
  list-style-type: none;
}
</style>
