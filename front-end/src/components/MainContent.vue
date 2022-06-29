<template>
  <n-card size="huge">
    <n-config-provider :hljs="hljs">
      <n-space>
        <n-code :code="cppCode" language="cpp" />
      </n-space>
      <n-space>
      <n-code
        :code="`
function sleep (ms = 1000) {
  return new Promise(resolve => setTimeout(resolve, ms))
}
`"
        language="javascript"
      />
      </n-space>
    </n-config-provider>
  </n-card>
</template>

<script setup lang="ts">
import hljs from "@/plugins/hljs"


let cppCode = `int main () {
  std::cout << "Hello Naive UI";
  return 0;
}`
</script>