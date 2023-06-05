import {
  ExecuteTransactionRequestType,
  SignedMessage,
  SignedTransaction,
  SuiTransactionBlockResponseOptions,
  TransactionBlock
} from '@mysten/sui.js'
import {
  SuiSignAndExecuteTransactionBlockInput,
  SuiSignAndExecuteTransactionBlockOutput,
  SuiSignMessageInput,
  SuiSignTransactionBlockInput
} from '@mysten/wallet-standard'
import { AppBaseInitialize, BaseApp } from 'base'
import { MessageToSign, TransactionToSign } from 'base/src/content'
import { SUI_NETWORK } from './utils'
export type AppSuiInitialize = Omit<AppBaseInitialize, 'network'>
export class AppSui {
  sessionId: string
  base: BaseApp

  constructor(base: BaseApp) {
    this.base = base
    this.sessionId = base.sessionId
  }

  public static build = async (initData: AppSuiInitialize): Promise<AppSui> => {
    const base = await BaseApp.build({ ...initData, network: SUI_NETWORK })
    return new AppSui(base)
  }
  signTransaction = async (input: SuiSignTransactionBlockInput) => {
    return await this.signTransactionBlock(input)
  }

  signTransactionBlock = async (
    input: SuiSignTransactionBlockInput
  ): Promise<SignedTransaction> => {
    const transactionToSign: TransactionToSign = {
      transaction: input.transactionBlock.serialize()
    }
    const signedTx = await this.base.signTransactions([transactionToSign])

    return JSON.parse(signedTx[0].transaction)
  }

  signMessage = async (input: SuiSignMessageInput, encoding?: string) => {
    const request: MessageToSign = {
      message: new TextDecoder().decode(input.message),
      metadata: JSON.stringify({ encoding: encoding || 'hex', account: input.account })
    }
    const signedTx = await this.base.signMessages([request])
    return JSON.parse(signedTx[0].message) as SignedMessage
  }

  // signAndExecuteTransactionBlock = async (input: SuiSignAndExecuteTransactionBlockInput): Promise<SuiSignAndExecuteTransactionBlockOutput> => {
  // }
}
