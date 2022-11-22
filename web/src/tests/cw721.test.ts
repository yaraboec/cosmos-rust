import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Token } from "../logic/types";
import { Cw721Contract } from "./../logic/cw721";

const signingClientMock = {
  instantiate: jest.fn(),
  execute: jest.fn(),
  queryContractSmart: jest.fn(),
};

const RECEIVER = "receiver";
const TOKEN_ID = "1";
const MINTER = "minter";
const TOKEN_URI = "ipfs12345";
const CONTRACT_ADDRESS = "wasm12345";
const RETURNED_CALL_RESULT = {
  transactionHash: "ABC123",
};
const DEFAULT_FUNDS = {
  amount: [{ amount: expect.any(String), denom: expect.any(String) }],
  gas: expect.any(String),
};

describe("cw721", () => {
  const cw721 = new Cw721Contract(
    CONTRACT_ADDRESS,
    signingClientMock as unknown as SigningCosmWasmClient
  );

  it("should proceed instantiation", async () => {
    const codeId = 123;
    signingClientMock.instantiate.mockResolvedValue({
      contractAddress: CONTRACT_ADDRESS,
    });

    const initRes = await Cw721Contract.instantiate(
      MINTER,
      signingClientMock as unknown as SigningCosmWasmClient,
      codeId
    );

    expect(signingClientMock.instantiate).toHaveBeenCalledWith(
      MINTER,
      codeId,
      { minter: MINTER, name: "contract", symbol: "some_symbol" },
      expect.any(String),
      DEFAULT_FUNDS
    );
    expect(initRes).toEqual(CONTRACT_ADDRESS);
  });

  it("should proceed minting", async () => {
    signingClientMock.execute.mockResolvedValue(RETURNED_CALL_RESULT);

    const mintTx = await cw721.mintToken(MINTER, RECEIVER, TOKEN_ID, TOKEN_URI);

    expect(signingClientMock.execute).toHaveBeenCalledWith(
      MINTER,
      CONTRACT_ADDRESS,
      {
        mint: {
          token: { owner: RECEIVER, token_id: TOKEN_ID, token_uri: TOKEN_URI },
        },
      },
      DEFAULT_FUNDS
    );
    expect(mintTx).toEqual(RETURNED_CALL_RESULT.transactionHash);
  });

  it("should proceed transferring", async () => {
    signingClientMock.execute.mockResolvedValue(RETURNED_CALL_RESULT);

    const transferTx = await cw721.transferToken(MINTER, TOKEN_ID, RECEIVER);

    expect(signingClientMock.execute).toHaveBeenCalledWith(
      MINTER,
      CONTRACT_ADDRESS,
      {
        transfer_nft: {
          to: RECEIVER,
          token_id: TOKEN_ID,
        },
      },
      DEFAULT_FUNDS
    );
    expect(transferTx).toEqual(RETURNED_CALL_RESULT.transactionHash);
  });

  it("should proceed querying tokens", async () => {
    const returnValue: Token[] = [
      { owner: MINTER, token_id: TOKEN_ID, token_uri: TOKEN_URI },
      { owner: MINTER, token_id: "2", token_uri: "ipfs54321" },
    ];
    signingClientMock.queryContractSmart.mockResolvedValue(returnValue);

    const tokens = await cw721.getOwnedTokens(MINTER);

    expect(signingClientMock.queryContractSmart).toHaveBeenCalledWith(
      CONTRACT_ADDRESS,
      {
        tokens: { owner: MINTER },
      }
    );
    expect(tokens).toEqual(returnValue);
  });
});
