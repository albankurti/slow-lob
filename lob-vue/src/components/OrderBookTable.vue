<template>
    <div class="asks-table">
      <table style="width: 50%, justify-self: end; align-self: center;">
        <tr v-for="(ask, index) in asks" :key="index">
          <td>{{ ask.ask }}</td>
          <td>{{ ask.askVolume }}</td>
        </tr>
      </table>
    </div>
    <div class="spread">
      Spread: {{ spread }}
    </div>
    <div class="bids-table">
      <table style="width: 50%, justify-self: end, align-self: center;">
        <tr v-for="(bid, index) in bids" :key="index">
          <td>{{ bid.bid }}</td>
          <td>{{ bid.bidVolume }}</td>
        </tr>
      </table>
    </div>
  </template>
  
  <script>
  export default {
    computed: {
      asks() {
        // Access the asks data from the store
        const asks = this.$store.state.ask_sequence;
        // Convert asks data to the specified format
        return asks.map(item => {
          return {
            ask: item.price,
            askVolume: item.volume
          };
        }).reverse();
      },
      bids() {
        // Access the bids data from the store
        const bids = this.$store.state.bid_sequence;
  
        // Convert bids data to the specified format
        return bids.map(item => {
          return {
            bid: item.price,
            bidVolume: item.volume
          };
        });
      },
      spread() {
        return Math.round(this.$store.state.spread * 100) / 100;
      }
    }
  };
  </script>
  

  <style>
    table {
        border-collapse: collapse;
        width: 100%;
    }

    th, td {
        border: 1px solid #ddd;
        text-align: center;
        padding: 8px;
    }

    th {
        background-color: #f2f2f2;
    }

    tr:nth-child(even) {
        background-color: #f2f2f2;
    }

    tr:hover {
        background-color: #ddd;
    }

    .spread {
        background-color: rgb(10,11,13); /* Set the background color of the entire table */
        color: gray;
        justify-content: center;
        align-content: center;
        align-items: center;
        justify-items: center;
        width: 100%;
        display: flex;
        flex-direction: column;
        height: 100%; /* Make the container take up the full height */
    }

    .asks-table {
    width: 100%; /* Set the table width to 100% to fill the container */
    background-color: rgb(10,11,13); /* Set the background color of the entire table */
    border-style: ridge;
    border-width: 3px;
    color: lightcoral;
    border-color: purple;
    }

    /* Apply alternate row background color for better readability */
    .asks-table tr:nth-child(even) {
    background-color: rgb(10,11,13); /* Set the background color for even rows */
    }

    .bids-table {
    width: 100%; /* Set the table width to 100% to fill the container */
    background-color: rgb(10,11,13); /* Set the background color of the entire table */
    border-style: ridge;
    border-width: 3px;
    border-color: purple;
    color: palegreen;
    }

    /* Apply alternate row background color for better readability */
    .bids-table tr:nth-child(even) {
    background-color: rgb(10,11,13); /* Set the background color for even rows */
    }
</style>