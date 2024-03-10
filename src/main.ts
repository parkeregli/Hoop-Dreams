import "./assets/main.css";

import { createApp } from "vue";
import { createPinia } from "pinia";

// UI Library
import PrimeVue from "primevue/config";

// Import global components
import Button from "primevue/button";
import Card from "primevue/card";
import OrderList from "primevue/orderlist";

import App from "./App.vue";
import router from "./router";
import "primeicons/primeicons.css";
import "primevue/resources/primevue.min.css";
import "primevue/resources/themes/aura-dark-lime/theme.css";

const app = createApp(App);

app.use(PrimeVue);

//Register global components
app.component("Button", Button);
app.component("Card", Card);
app.component("OrderList", OrderList);

app.use(createPinia());
app.use(router);

app.mount("#app");
