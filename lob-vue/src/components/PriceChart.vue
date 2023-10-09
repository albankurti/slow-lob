<template>
  <websocket-data/>
  <Line
    id="my-chart-id"
    :options="chartOptions"
    :data="chartData"
    :plugins="chartOptions.plugins"
  />
</template>

<script>
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
} from 'chart.js'
import { Line } from 'vue-chartjs'
import WebsocketData from '../utils/WebsocketData.vue'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
)

const getDates = () => {
  let labels = []
  for (let i = -60; i <= 10; i++) {
    const time = new Date(Date.now() + i * 60 * 1000); // Add/subtract minutes
    const minutes = time.getMinutes().toString().padStart(2, '0');
    const hours = time.getHours().toString().padStart(2, '0');
    labels.push(i);
  }
  return labels;
}

const custom_canvas_background_color = {
      id: 'custom_canvas_background_color',
      beforeDraw: (chart, args, options) => {
        const {
          ctx,
          chartArea: { top, right, bottom, left, width, height },
          scales: { x, y },
        } = chart;
        ctx.save();
        ctx.globalCompositeOperation = 'destination-over';
        ctx.fillStyle = 'rgb(10,11,13)';
        ctx.fillRect(left, top, width, height);
        ctx.restore();
      },
    };

export default {
  name: 'PriceChart',
  components: { Line, WebsocketData },
  computed: {
    chartData() {
      return{
        labels: getDates(),
        datasets: [
          {
            label: 'Ask',
            borderColor: '#FF033E',
            borderWidth: '1',
            data: [...this.$store.state.best_asks].slice(-60)
          },
          {
            label: 'Bid',
            borderColor: '#20B384',
            borderWidth: '1',
            data: [...this.$store.state.best_bids].slice(-60)
          }
        ]
      }
    },
    chartOptions() {
      const max = Math.max(...this.$store.state.best_asks, ...this.$store.state.best_bids);
      const min = Math.min(...this.$store.state.best_asks, ...this.$store.state.best_bids);
      
      return {
        responsive: true,
        chartArea:{
          backgroundColor: 'red'
        },
        scales: {
          x: {
            grid: {
              color: '#303030', // Light gray color for grid lines
            },
          },
          y: {
            grid: {
              color: '#303030', // Light gray color for grid lines
            },
            max: max * 1.0000001,
            min: min * 0.9999999,
          },
        },
        plugins: [custom_canvas_background_color],
        interaction: {
          intersect: false
        },
        animation: {
          duration: this.animationEnabled ? 1000 : 0, // Set animation duration based on the animationEnabled flag
        },
      };
    }
  },
};
</script>