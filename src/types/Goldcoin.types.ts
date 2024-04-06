export type Addr = string;
export type Uint128 = string;
export interface InstantiateMsg {
  _admin: Addr;
  _decimals: number;
  _denom: string;
  _exchange_rate: Uint128;
  _initial_supply: Uint128;
  _name: string;
  _symbol: string;
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
export type QueryMsg = {
  balance_of: {
    addr: Addr;
  };
} | {
  get_total_supply: {};
} | {
  get_exchange_rate: {};
};