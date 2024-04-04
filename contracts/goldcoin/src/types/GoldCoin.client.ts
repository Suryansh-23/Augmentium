/**
* This file was automatically generated by @oraichain/ts-codegen@0.35.9.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @oraichain/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import {Addr, Uint128, InstantiateMsg, ExecuteMsg, QueryMsg} from "./GoldCoin.types";
export interface GoldCoinReadOnlyInterface {
  contractAddress: string;
  balanceOf: ({
    addr
  }: {
    addr: Addr;
  }) => Promise<BalanceOfResponse>;
  allowance: ({
    owner,
    spender
  }: {
    owner: Addr;
    spender: Addr;
  }) => Promise<AllowanceResponse>;
  getTotalSupply: () => Promise<GetTotalSupplyResponse>;
  getExchangeRate: () => Promise<GetExchangeRateResponse>;
}
export class GoldCoinQueryClient implements GoldCoinReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.balanceOf = this.balanceOf.bind(this);
    this.allowance = this.allowance.bind(this);
    this.getTotalSupply = this.getTotalSupply.bind(this);
    this.getExchangeRate = this.getExchangeRate.bind(this);
  }

  balanceOf = async ({
    addr
  }: {
    addr: Addr;
  }): Promise<BalanceOfResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      balance_of: {
        addr
      }
    });
  };
  allowance = async ({
    owner,
    spender
  }: {
    owner: Addr;
    spender: Addr;
  }): Promise<AllowanceResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      allowance: {
        owner,
        spender
      }
    });
  };
  getTotalSupply = async (): Promise<GetTotalSupplyResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_total_supply: {}
    });
  };
  getExchangeRate = async (): Promise<GetExchangeRateResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_exchange_rate: {}
    });
  };
}
export interface GoldCoinInterface extends GoldCoinReadOnlyInterface {
  contractAddress: string;
  sender: string;
  transfer: ({
    amount,
    recipient
  }: {
    amount: Uint128;
    recipient: Addr;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  approve: ({
    amount,
    spender
  }: {
    amount: Uint128;
    spender: Addr;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  setExchangeRate: ({
    exchangeRate
  }: {
    exchangeRate: Uint128;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  transferFrom: ({
    amount,
    recipient,
    sender
  }: {
    amount: Uint128;
    recipient: Addr;
    sender: Addr;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  buyGC: (_fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  redeemGC: ({
    gcAmount
  }: {
    gcAmount: Uint128;
  }, _fee?: number | StdFee | "auto", _memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class GoldCoinClient extends GoldCoinQueryClient implements GoldCoinInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.transfer = this.transfer.bind(this);
    this.approve = this.approve.bind(this);
    this.setExchangeRate = this.setExchangeRate.bind(this);
    this.transferFrom = this.transferFrom.bind(this);
    this.buyGC = this.buyGC.bind(this);
    this.redeemGC = this.redeemGC.bind(this);
  }

  transfer = async ({
    amount,
    recipient
  }: {
    amount: Uint128;
    recipient: Addr;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      transfer: {
        amount,
        recipient
      }
    }, _fee, _memo, _funds);
  };
  approve = async ({
    amount,
    spender
  }: {
    amount: Uint128;
    spender: Addr;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      approve: {
        amount,
        spender
      }
    }, _fee, _memo, _funds);
  };
  setExchangeRate = async ({
    exchangeRate
  }: {
    exchangeRate: Uint128;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_exchange_rate: {
        exchange_rate: exchangeRate
      }
    }, _fee, _memo, _funds);
  };
  transferFrom = async ({
    amount,
    recipient,
    sender
  }: {
    amount: Uint128;
    recipient: Addr;
    sender: Addr;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      transfer_from: {
        amount,
        recipient,
        sender
      }
    }, _fee, _memo, _funds);
  };
  buyGC = async (_fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      buy_g_c: {}
    }, _fee, _memo, _funds);
  };
  redeemGC = async ({
    gcAmount
  }: {
    gcAmount: Uint128;
  }, _fee: number | StdFee | "auto" = "auto", _memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      redeem_g_c: {
        gc_amount: gcAmount
      }
    }, _fee, _memo, _funds);
  };
}