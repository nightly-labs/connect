---
title: Modal UI overrides
slug: customization/ui_overrides
---

Nightly connect offers a default modal that comes along with the adapter, so that you don't have to put any additional work into implementing it yourself. Nevertheless, if you wish to do so, you can.

There are two ways of customizing the modal, one of which is UI overrides and the other is implementing an [external modal](./external_modal).

### UI overrides

Customizing with the use of UI overrides is easier than creating the external modal, but it guarantees only limited freedom in the customization options.

The overrides is a set of properties, that make up an object, which then is optionally passed into the `build()` or `buildLazy()` functions. The properties look like this.

```js
interface UIOverrides {
    variablesOverride?: object
    stylesOverride?: string
    qrConfigOverride?: Partial<XMLOptions>
}

interface XMLOptions {
    image?: string;
    imageWidth?: number;
    imageHeight?: number;
    width: number;
    height: number;
    margin: number;
    data: string;
    qrOptions: {
        typeNumber: TypeNum;
        mode?: Mode;
        errorCorrectionLevel: ErrorCorrectionLevel;
    };
    imageOptions: {
        hideBackgroundDots: boolean;
        imageSize: number;
        crossOrigin?: string;
        margin: number;
    };
    dotsOptions: {
        color: string;
    };
    cornersDotOptions: {
        color: string;
    };
    cornersSquareOptions: {
        color: string;
    };
    backgroundOptions: {
        color: string;
    };
}

type TypeNum = Range<0, 41>;

enum Mode {
    Numeric = "Numeric",
    Alphanumeric = "Alphanumeric",
    Byte = "Byte",
    Kanji = "Kanji"
}

enum ErrorCorrectionLevel {
    L = "L",
    M = "M",
    Q = "Q",
    H = "H"
}
```

:::info
The `XMLOptions` interface, specifies the override object for the QR code, which is displayed on the modal.
:::

As you can see, the options are plentiful and allow for great flexibility in customizing the appearance of the modal.
Below is the example of implementing the overrides.

```js
const adapter = NightlyConnectAdapter.buildLazy(
  {
    appMetadata: {
      name: 'NC TEST AlephZero',
      description: 'Nightly Connect Test',
      icon: 'https://docs.nightly.app/img/logo.png',
      additionalInfo: 'Courtesy of Nightly Connect team'
    },
    network: 'AlephZero'
  },
  {
    variablesOverride: {
      '--nc-color-primary': 'green',
      '--nc-img-logo': 'url(https://alephzero.org/aleph-design/brand-elements/logo-day.svg)'
    }, // override the CSS variables
    stylesOverride: `
        .nc_headerWrapper {
            background-color: red;
        }

        .nc_headerLogo {
            width: 200px;
        }

        .nc_modalContent {
            border-radius: 0;
            border: 3px dashed var(--nc-color-primary);
        }
        `,
    // override the styles manually
    qrConfigOverride: {
      image: customFennecXml,
      dotsOptions: {
        color: 'gold'
      }
    }
    // override the qr code cinfiguration
  }
)
```

:::info
The example is built using [Substrate](../../substrate/substrate/connect), but can be implemented using [Solana](../../solana/solana/connect) and [SUI](../../sui/sui/connect) as well.
:::
