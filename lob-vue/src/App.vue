<template>
  <div id="app" class="app-container">
    <div>
      <Dashboard/>
    </div>
    <div style="width: 80%; float: left;"> 
      <PriceChart/>
    </div>
    <div style="margin-left: 80%;">
      <OrderBookTableVue/>
    </div>
  </div>
</template>

<script>
import WebsocketData from './utils/WebsocketData.vue';
import PriceChart from './components/PriceChart.vue';
import OrderBookTableVue from './components/OrderBookTable.vue';
import Dashboard from './components/Dashboard.vue';

export default {
  name: 'App',
  components: { PriceChart, WebsocketData, OrderBookTableVue },
}
</script>

,
    Dashboard<style>
/* Apply background color to the whole page */
.app-container {
  background-color: rgb(10,11,13);
  width: 101%;
  height: 101vh; /* Use 100vh to make it cover the full viewport height */
}

/* Other styles for your components and elements */
</style>
