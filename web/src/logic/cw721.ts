import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Token } from "./types";

export class Cw721Contract {
  contractAddress: string;
  signingClient: SigningCosmWasmClient;

  constructor(contractAddress: string, signingClient: SigningCosmWasmClient) {
    this.contractAddress = contractAddress;
    this.signingClient = signingClient;
  }

  static async instantiate(
    senderAddress: string,
    signingClient: SigningCosmWasmClient,
    codeId: number
  ) {
    const { contractAddress } = await signingClient.instantiate(
      senderAddress,
      codeId,
      {
        name: "contract",
        symbol: "some_symbol",
        minter: senderAddress,
      },
      Math.random().toString(20).substring(2, 6),
      {
        gas: "200000",
        amount: [{ denom: "umlg", amount: "5000" }],
      }
    );

    return contractAddress;
  }

  async getOwnedTokens(ownerAddress: string): Promise<Array<Token>> {
    const ownedTokens = await this.signingClient.queryContractSmart(
      this.contractAddress,
      { tokens: { owner: ownerAddress } }
    );

    return ownedTokens;
  }

  async mintToken(
    senderAddress: string,
    owner: string,
    tokenId: string,
    tokenUri?: string
  ): Promise<string> {
    console.log(senderAddress, owner, this.contractAddress);
    const mintRes = await this.signingClient.execute(
      senderAddress,
      this.contractAddress,
      {
        mint: {
          token: {
            owner,
            token_id: tokenId,
            token_uri: tokenUri,
          },
        },
      },
      {
        gas: "200000",
        amount: [{ denom: "umlg", amount: "10000" }],
      }
    );

    return mintRes.transactionHash;
  }

  async transferToken(
    senderAddress: string,
    tokenId: string,
    receiver: string
  ): Promise<string> {
    const transferRes = await this.signingClient.execute(
      senderAddress,
      this.contractAddress,
      {
        transfer_nft: {
          token_id: tokenId,
          to: receiver,
        },
      },
      {
        gas: "200000",
        amount: [{ denom: "umlg", amount: "5000" }],
      }
    );

    return transferRes.transactionHash;
  }
}
