<script lang="ts">
import { defineComponent, ref } from "vue";

import init, {
  getState,
  openField,
  toggleFlag,
  resetGame,
  resetChangeMineNumber,
} from "../../../pkg/minesweeper_wasm_rust.js";
const component = defineComponent({
  name: "mine-sweeper",
  setup() {
    // Initialised variable to make sure wasm is initialised
    let ready = ref(false);
    // Initialised variable to update data at each click
    let data = ref([] as string[][]);
    return { ready, data };
  },
  data() {
    return {
      mines: 5,
    };
  },
  async mounted() {
    await init();
    this.ready = true;
    this.update_data();
  },
  methods: {
    left_click_manager(x: number, y: number): void {
      openField(x, y);
      this.update_data();
    },
    right_click_manager(x: number, y: number): void {
      toggleFlag(x, y);
      this.update_data();
    },
    update_data() {
      this.data = getState()
        .split("\n")
        .map((row) => row.trim().split(/\s+/));
    },
    reset(): void {
      resetGame();
      this.update_data();
    },
    reset_change_mine_number(mine_count: number): void {
      resetChangeMineNumber(mine_count);
      this.update_data();
    },
  },
});

export default component;
</script>

<template>
  <div>
    <link
      href="https://unpkg.com/tailwindcss@^1.0/dist/tailwind.min.css"
      rel="stylesheet"
    />
    <h2 class="font-medium leading-tight text-4xl mt-0 mb-2 text-blue-600">
      MineSweeper WASM Rust Master Race
    </h2>
    <div v-if="ready" id="root">
      <div v-for="(d, y) in data" :key="d">
        <div
          class="field"
          v-for="(e, x) in d"
          :key="e"
          @click="left_click_manager(x, y)"
          @contextmenu="right_click_manager(x, y)"
        >
          {{ e }}
        </div>
      </div>
    </div>
    <div>
      <div>Mines count: {{ mines }}</div>

      <select v-model="mines">
        <option disabled value="">Please select mine count</option>
        <option>5</option>
        <option>10</option>
        <option>20</option>
        <option>40</option>
        <option>80</option>
      </select>
    </div>
    <button
      id="reset_change_mine_number"
      class="bg-blue-500 hover:bg-blue-700 text-white py-1 px-2 rounded-full"
      style="font-size: 18px"
      @click="reset_change_mine_number(this.mines)"
    >
      Reset with {{ mines }} mines
    </button>
  </div>
</template>

<style>
html {
  font-size: 200%;
  font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
}

.field {
  text-decoration: none;
  text-align: center;
  width: 1.2rem;
  height: 1.2rem;
  line-height: 1.2rem;
  cursor: pointer;
}
#root {
  display: inline-grid;
  grid-template: repeat(10, auto) / repeat(10, auto);
}
#reset {
  border: 1px solid black;
  cursor: pointer;
}
</style>