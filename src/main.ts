import "./assets/main.css";

import { createApp } from "vue";
import { createPinia } from "pinia";

// UI Library
import PrimeVue from "primevue/config";

// Import global components
import Button from "primevue/button";
import Card from "primevue/card";
import DataView from "primevue/dataview";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Toolbar from "primevue/toolbar";
import SplitButton from "primevue/splitbutton";
import Menu from "primevue/menu";
import ToggleButton from "primevue/togglebutton";
import SelectButton from "primevue/selectbutton";

import App from "./App.vue";
import router from "./router";
import "primeicons/primeicons.css";
import "primevue/resources/primevue.min.css";
import "primevue/resources/themes/aura-dark-lime/theme.css";
import "primeflex/primeflex.css";
const app = createApp(App);

app.use(PrimeVue);

//Register global components
app.component("Button", Button);
app.component("Card", Card);
app.component("DataView", DataView);
app.component("DataTable", DataTable);
app.component("Column", Column);
app.component("Toolbar", Toolbar);
app.component("SplitButton", SplitButton);
app.component("ToggleButton", ToggleButton);
app.component("SelectButton", SelectButton);
app.component("Menu", Menu);
app.use(createPinia());
app.use(router);

app.mount("#app");
