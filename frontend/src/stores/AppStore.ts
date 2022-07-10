import { defineStore } from "pinia";

export const useAppStore = defineStore({
    id: "app",
    state: () => ({
        test: "" as string
    }),
    getters: {
        getTest(): string {
            return this.test;
        }
    },
    actions: {
        setTest(value: string) {
            this.test = value;
        }
    },
});