<template>
  <nav>
    <router-link to="/" class="link">Home</router-link>
    <router-link to="/jisho" class="link">API</router-link>
  </nav>
  <router-view/>
</template>

<style lang="scss">
@import "@/assets/variables.scss";

$nav-height: 30px;

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}

nav {
    display: flex;
    justify-content: space-evenly;
    position: absolute;
    bottom: 0px;
    background-color: $gray;
    width: 100vw;
    height: $nav-height;
    color: white;
    line-height: 30px;
    padding: 10px 0px;

    .link {
        color: white;
        text-decoration: none;

        &.router-link-exact-active {
            color: $primary-color;
        }
    }
}
</style>
