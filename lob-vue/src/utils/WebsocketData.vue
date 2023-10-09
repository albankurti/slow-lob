<template>
  <div>
    <!-- Your template content goes here -->
  </div>
</template>

<script>

export default {
  data() {
    return {
      message: '',
      messages: [],
      socket: null,
    };
  },
  methods: {
    initializeWebSocket() {
      this.socket = new WebSocket('ws://127.0.0.1:8085/websocket');

      this.socket.onopen = () => {
        console.log('WebSocket connection opened');
      };

      this.socket.onerror = (error) => {
        console.error('WebSocket error:', error);
      };

      this.socket.onclose = (event) => {
        if (event.wasClean) {
          console.log(`WebSocket connection closed cleanly, code=${event.code}, reason=${event.reason}`);
        } else {
          console.error(`WebSocket connection abruptly closed`);
          console.error(event);
        }

        // Reconnect after a short delay (you can customize the delay)
        setTimeout(this.initializeWebSocket, 1000); // Reconnect after 1 second
      };

      this.socket.onmessage = (event) => {
        // Split the input into ask and bid
        const [askString, bidString] = event.data.split("], [");

        // Extract ask_sequence and bid_sequence
        const askMatch = askString.match(/\(([^,]+), Limit { limit_price: ([^,]+), total_volume: ([^,]+)/g);
        const bidMatch = bidString.match(/\(([^,]+), Limit { limit_price: ([^,]+), total_volume: ([^,]+)/g);

        // Process ask_sequence and bid_sequence into arrays
        const ask_sequence = askMatch.map(match => {
          const [, price, , volume] = match.match(/\(([^,]+), Limit { limit_price: ([^,]+), total_volume: ([^,]+)/);
          return { price: parseFloat(price), volume: parseFloat(volume) };
        });

        const bid_sequence = bidMatch.map(match => {
          const [, price, , volume] = match.match(/\(([^,]+), Limit { limit_price: ([^,]+), total_volume: ([^,]+)/);
          return { price: parseFloat(price), volume: parseFloat(volume) };
        });

        this.$nextTick( () => {
          this.$store.commit('update', [ask_sequence[0].price, 
            bid_sequence[0].price]);
          this.$store.commit('update_table', [ask_sequence, bid_sequence]);
        })
      };
    },
  },
  mounted() {
    this.initializeWebSocket();
  },
};
</script>
