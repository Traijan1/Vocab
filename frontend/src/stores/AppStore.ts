import { AppView } from "@/models/AppView";
import { defineStore } from "pinia";

export const useAppStore = defineStore({
    id: "app",
    state: () => ({
        currentView: AppView.Vocab
    }),
    getters: {
        getView(): AppView {
            return this.currentView;
        }
    },
    actions: {
        setView(value: AppView) {
            this.test = value;
        }
    },
});