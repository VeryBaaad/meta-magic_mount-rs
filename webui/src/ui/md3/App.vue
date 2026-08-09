<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import Md3Layout from "./Md3Layout.vue";
import StatusPage from "./pages/status.vue";
import ConfigPage from "./pages/config.vue";
import ModulesPage from "./pages/modules.vue";
import AboutPage from "./pages/about.vue";

const { t } = useI18n();
const pages = [StatusPage, ConfigPage, ModulesPage, AboutPage];
const navindex = ref(0);
const activepage = computed(() => pages[navindex.value]);
const titles = computed(() => [
  t("tabs.status"),
  t("tabs.config"),
  t("tabs.modules"),
  t("tabs.info"),
]);
</script>

<template>
  <Md3Layout
    v-model:navindex="navindex"
    :activepage="activepage"
    :titles="titles"
  />
</template>
