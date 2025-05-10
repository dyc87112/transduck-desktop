import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import App from "./App.vue";
import "bootstrap/dist/css/bootstrap.css";

// 导入语言文件
import en from "./locales/en.json";
import zh from "./locales/zh.json";

// 创建i18n实例
const i18n = createI18n({
  legacy: false, // 使用Composition API
  locale: localStorage.getItem("locale") || navigator.language.split("-")[0] || "zh",
  fallbackLocale: "en",
  messages: {
    en,
    zh
  }
});

// 创建应用实例
const app = createApp(App);

// 使用插件
app.use(i18n);

// 挂载应用
app.mount("#app");