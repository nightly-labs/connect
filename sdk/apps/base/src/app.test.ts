import { assert, describe, expect, test } from 'vitest'
import { BaseApp } from './app'
import { testAppBaseInitialize } from './utils'

// Edit an assertion and save to see HMR in action

describe('Base App tests', () => {
  test('#Build()', async () => {
    const baseApp = await BaseApp.build(testAppBaseInitialize)
    expect(baseApp).toBeDefined()
    assert(baseApp.sessionId !== '')
  })
})
