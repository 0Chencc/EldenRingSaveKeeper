<script setup>
const params = new URLSearchParams(window.location.search)
const index = params.get('index')
</script>

<template>
  <div><p>
    {{ params }}
  </p></div>
</template>

<style scoped>

</style>