export type Uint128 = string;
export interface InstantiateMsg {
  decimals: number;
  denom: string;
  exchange_rate: Uint128;
  initial_supply: Uint128;
  name: string;
  symbol: string;
}
export type ExecuteMsg = {
  transfer: {
    amount: Uint128;
    recipient: Addr;
  };
} | {
  set_exchange_rate: {
    exchange_rate: number;
  };
} | {
  buy: {};
} | {
  redeem: {
    gc_amount: Uint128;
  };
};
export type Addr = string;
export type QueryMsg = {
  balance_of: {
    addr: Addr;
  };
} | {
  get_total_supply: {};
} | {
  get_exchange_rate: {};
};