<template>
  <Card class="about">
    <template #title>
      <h1>About</h1>
    </template>
    <template #content>
      <h1>This is an about page</h1>
    </template>
  </Card>
</template>

<style>
@media (min-width: 1024px) {
  .about {
    min-height: 100vh;
    display: flex;
    align-items: center;
  }
}
</style>
