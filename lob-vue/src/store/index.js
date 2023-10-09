import { createStore } from "vuex";

export default createStore({
    state: {
        best_bids: [],
        best_asks: [],
        ask_sequence: [],
        bid_sequence: [],
        spread: Number
    },
    mutations: {
        update(state, data){
            let ask = data[0];
            let bid = data[1];
            if (state.best_bids[state.best_bids.length - 1] !== bid || state.best_asks[state.best_asks.length - 1] !== ask
                || state.best_asks.length < 40 || state.best_bids.length < 40){
                state.best_asks.push(ask);
                state.best_bids.push(bid);
            }
            if (state.best_bids.length > 40){
                state.best_bids = state.best_bids.slice(-40);
                state.best_asks = state.best_asks.slice(-40);
            }
            state.spread = ask - bid;
        },
        update_table(state, data){
            let ask_sequence = data[0];
            let bid_sequence = data[1];
            state.ask_sequence = ask_sequence;
            state.bid_sequence = bid_sequence;
        }
    },
    actions: {

    },
})