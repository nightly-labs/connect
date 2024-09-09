pub static RESET_PASSWORD_TEMPLATE: &str = r##"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Forgotten password</title>
    <style>
      @import url("https://fonts.googleapis.com/css2?family=Poppins:wght@300&display=swap");
      @import url("https://fonts.googleapis.com/css2?family=Poppins:wght@700&display=swap");

      @media screen {
        @font-face {
          font-family: "Poppins";
          font-weight: 400;
          font-style: normal;
          src: local("Poppins"), local("sans-serif"),
            url("https://fonts.googleapis.com/css2?family=Poppins:wght@300&display=swap");
        }
        @font-face {
          font-family: "Poppins";
          font-weight: 700;
          font-style: normal;
          src: local("Poppins"), local("sans-serif"),
            url("https://fonts.googleapis.com/css2?family=Poppins:wght@700&display=swap");
        }
      }

      table {
        border-collapse: collapse !important;
        margin: 0 auto;
      }
      td {
        vertical-align: middle;
      }
      button {
        align-items: center !important;
      }
    </style>
  </head>
  <body>
    <table
      width="100%"
      cellpadding="0"
      cellspacing="0"
      style="background-color: #171c2f; max-width: 640px; height: 156px"
    >
      <tr>
        <td>
          <img
            style="width: 100%"
            src="https://registry.nightly.app/email/img/banner.png"
            alt=""
          />
        </td>
      </tr>
    </table>
    <table
      width="100%"
      cellpadding="0"
      cellspacing="0"
      style="
        background-color: #ffffff;
        padding: 0 64px;
        max-width: 640px;
        border-bottom: #b1bdd4 1px solid;
      "
    >
      <tr>
        <td style="padding: 0 64px">
          <h1
            style="
              font-family: 'Poppins', sans-serif;
              font-size: 36px;
              font-weight: 800;
              margin-top: 23px;
              margin-bottom: 25px;
              color: #040407;
            "
          >
            Forgotten password
          </h1>
          <p
            style="
              font-family: 'Poppins', sans-serif;
              font-size: 16px;
              font-weight: 400;
              line-height: 22px;
              margin-top: 0;
              margin-bottom: 32px;
            "
          >
            We noticed that you forgot your password. No worries, we're here to
            help you regain access to your account!
          </p>
          <p
            style="
              font-family: 'Poppins', sans-serif;
              font-size: 16px;
              font-weight: 400;
              line-height: 22px;
              margin-top: 0;
              margin-bottom: 32px;
            "
          >
            To create a new password, click the button below:
          </p>
          <tr>
            <td align="left" bgcolor="#ffffff">
              <table border="0" cellpadding="0" cellspacing="0" width="100%">
                <tr>
                  <td
                    align="center"
                    bgcolor="#ffffff"
                    width="100%"
                    style="padding: 0 0 32px 0"
                  >
                    <table border="0" cellpadding="0" cellspacing="0">
                      <tr>
                        <td
                          align="center"
                          bgcolor="#6067F9"
                          style="border-radius: 6px"
                        >
                          <a
                            href="RESET_PASSWORD_LINK_TO_REPLACE"
                            target="_blank"
                            style="
                              display: inline-block;
                              padding: 9px 0;
                              font-size: 16px;
                              color: #ffffff;
                              text-decoration: none;
                              border-radius: 6px;
                              width: 512px;
                            "
                          >
                            Change password</a
                          >
                        </td>
                      </tr>
                    </table>
                  </td>
                </tr>
              </table>
            </td>
          </tr>
        </td>
      </tr>
    </table>
    <table
      width="100%"
      cellpadding="0"
      cellspacing="0"
      style="margin-top: 32px; max-width: 640px; background-color: #ffffff"
    >
      <tr>
        <td style="padding: 0 64px">
          <p
            style="
              margin: 0;
              font-size: 16px;
              font-weight: 400;
              line-height: 22px;
              font-family: 'Poppins', sans-serif;
            "
          >
            Button not working? Try the verification link:
          </p>
          <a
            id="link"
            style="
              font-family: 'Poppins', sans-serif;
              font-size: 16px;
              font-weight: 400;
              line-height: 22px;
              color: #6067f9;
              text-decoration: none;
              word-break: break-word;
            "
            href="RESET_PASSWORD_LINK_TO_REPLACE"
            >RESET_PASSWORD_LINK_TO_REPLACE</a
          >
          <p
            style="
              font-family: 'Poppins', sans-serif;
              color: #d25858;
              margin-top: 16px;
              font-size: 14px;
              font-weight: 400;
              line-height: 20px;
              margin-bottom: 32px;
            "
          >
            If you didn't request this password reset, please contact our
            support team and secure your crypto wallet immediately.
          </p>
        </td>
      </tr>
    </table>
    <table
      width="100%"
      cellpadding="0"
      cellspacing="0"
      style="
        height: 132px;
        max-width: 640px;
        background-color: #0f0f1a;
        font-family: 'Poppins', sans-serif;
      "
    >
      <tr>
        <td align="center">
          <h2
            style="
              font-family: 'Poppins', sans-serif;
              color: #f7f7f7;
              font-weight: 700;
              font-size: 22px;
              line-height: 30px;
              margin: 16px 0 8px 0;
            "
          >
            Stay in touch!
          </h2>
          <table
            align="center"
            border="0"
            cellpadding="0"
            cellspacing="0"
            width="100%"
            style="height: 22px; margin-bottom: 24px"
          >
            <tr>
              <td bgcolor="#0f0f1a" style="border-radius: 6px">
                <table align="center">
                  <tr>
                    <td>
                      <a
                        href="https://discord.com/invite/7nhFHA6yZq"
                        target="_blank"
                        style="
                          font-family: 'Source Sans Pro', Helvetica, Arial,
                            sans-serif;
                          font-size: 16px;
                          color: #ffffff;
                          text-decoration: none;

                          gap: 8px;
                        "
                      >
                        <img
                          style="
                            width: 24px;
                            height: 18px;
                            margin-bottom: 4px;
                            vertical-align: middle;
                          "
                          src="https://registry.nightly.app/email/img/discordIcon.png"
                          alt="discordIcon"
                        />
                        <span>Discord</span>
                      </a>
                    </td>

                    <td>
                      <a
                        href="https://discord.com/invite/7nhFHA6yZq"
                        target="_blank"
                        style="
                          font-family: 'Source Sans Pro', Helvetica, Arial,
                            sans-serif;
                          margin-left: 24px;
                          font-size: 16px;
                          color: #ffffff;
                          text-decoration: none;
                          gap: 8px;
                        "
                      >
                        <img
                          style="
                            width: 24px;
                            height: 18px;
                            margin-bottom: 4px;
                            vertical-align: middle;
                          "
                          src="https://registry.nightly.app/email/img/twitterIcon.png"
                          alt="twitterIcon"
                        />
                        <span>Twitter</span>
                      </a>
                    </td>
                  </tr>
                </table>
              </td>
            </tr>
          </table>
          <p
            style="
              font-family: 'Poppins', sans-serif;
              margin-top: 0;
              margin-bottom: 16px;
              color: #3e4864;
              font-size: 12px;
              font-weight: 400;
              line-height: 16px;
              margin-bottom: 16px;
            "
          >
            ©2023 - Nightly Exchange. All rights reserved.
          </p>
        </td>
      </tr>
    </table>
  </body>
</html>
"##;