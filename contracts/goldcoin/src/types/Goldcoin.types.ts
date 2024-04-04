export type Addr = string;
export type Uint128 = string;
export interface InstantiateMsg {
  _admin?: string | null;
  _asset: Addr;
  _decimals: number;
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
  approve: {
    amount: Uint128;
    spender: Addr;
  };
} | {
  set_exchange_rate: {
    exchange_rate: Uint128;
  };
} | {
  transfer_from: {
    amount: Uint128;
    recipient: Addr;
    sender: Addr;
  };
} | {
  buy_g_c: {};
} | {
  redeem_g_c: {
    gc_amount: Uint128;
  };
};
export type QueryMsg = {
  balance_of: {
    addr: Addr;
  };
} | {
  allowance: {
    owner: Addr;
    spender: Addr;
  };
} | {
  get_total_supply: {};
} | {
  get_exchange_rate: {};
};