<template>
    <n-grid cols="4">
        <n-gi>
            <n-select v-model:value="tagValueRef" :options="tagOptionsRef" class="w-48" />
        </n-gi>
        <n-gi span="2">
            <n-auto-complete v-model:value="catalogValueRef" :input-props="{
                autocomplete: 'disabled'
            }" :options="catalogOptionsFunc" placeholder="邮箱" />
        </n-gi>

        <n-gi>
            <n-dropdown trigger="hover" :options="menuOptions" @select="handleSelect">
                <n-button>Menu</n-button>
            </n-dropdown>
        </n-gi>
    </n-grid>
</template>
<script setup lang="ts">
import { ref, computed } from "vue"
import { useMessage } from 'naive-ui'

let tagValueRef = ref("");
let tagOptionsRef = ref([
    {
        label: "Everybody's Got Something to Hide Except Me and My Monkey",
        value: 'song0',

    },
]);
let catalogValueRef = ref("");
let catalogOptionsFunc = computed(() => {
    return ['@gmail.com', '@163.com', '@qq.com'].map((suffix) => {
        const prefix = catalogValueRef.value.split('@')[0]
        return {
            label: prefix + suffix,
            value: prefix + suffix
        }
    })
});

let menuOptions = [
    {
        label: '滨海湾金沙，新加坡',
        key: 'marina bay sands',
        disabled: true
    },
    {
        label: '布朗酒店，伦敦',
        key: "brown's hotel, london"
    },];

const message = useMessage()
const handleSelect = (key: string | number) => {
    message.info(String(key))
}
</script>
<style scoped>
</style>