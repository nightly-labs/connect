pub static TEAM_INVITE_NOTIFICATION_TEMPLATE: &str = r##"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Team invite</title>
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
      a:hover {
        opacity: 0.5;
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
      style="
        background-color: #f7f7f7;
        max-width: 640px;
        display: flex;
        flex-direction: column;
      "
    >
      <tr>
        <td style="padding: 64px">
          <img
            style="height: 48px"
            src="https://registry.nightly.app/email/img/ncCloudSmall.png"
            alt=""
          />
          <h1
            style="
              font-family: 'Poppins', sans-serif;
              font-size: 36px;
              font-weight: 700;
              margin-top: 32px;
              color: #040407;
              line-height: 44px;
            "
          >
            Welcome to the team!
          </h1>
          <p
            style="
              font-family: 'Poppins', sans-serif;
              font-size: 16px;
              font-weight: 400;
              line-height: 22px;
              margin-top: 32px;
              color: #2b344d;
            "
          >
            You just got invited to the special group of people EMAIL_TEAM_NAME!
            Are You willing to accept this invitation? We hope so, click link
            below to log and say yes!
          </p>
          <a
            href="EMAIL_TEAM_LINK"
            style="
              font-family: 'Poppins', sans-serif;
              font-size: 36px;
              font-weight: 900;
              margin-top: 16px;
              color: #040407;
              word-break: break-all;
              line-height: 44px;
            "
          >
            EMAIL_TEAM_LINK
          </a>
          <p
            style="
              font-family: 'Poppins', sans-serif;
              font-size: 12px;
              font-weight: 400;
              margin-top: 32px;
              line-height: 16px;
              color: #d25858;
            "
          >
            The invitation has no expiry date, but do not make your crew wait
            too long for You to join them!
          </p>
          <h3
            style="
              font-family: 'Poppins', sans-serif;
              font-size: 16px;
              font-weight: 700;
              line-height: 22px;
              margin-top: 32px;
              color: #2b344d;
            "
          >
            <span style="font-weight: 400"
              >We wish you great time creating together,</span
            >
            <br />Nightly's Team
          </h3>
          <div>
            <h4
              style="
                font-family: 'Poppins', sans-serif;
                font-size: 12px;
                font-weight: 400;
                line-height: 16px;
                margin-top: 32px;
                color: #2b344d;
              "
            >
              Hit us up here
            </h4>
            <div>
              <a href="https://x.com/Nightly_app" target="_blank">
                <img
                  style="height: 14px; margin-bottom: 1px"
                  src="https://registry.nightly.app/email/img/X.png"
                  alt=""
                />
              </a>
              <a
                href="https://discord.com/invite/7nhFHA6yZq"
                target="_blank"
                style="margin-left: 15px"
              >
                <img
                  style="height: 16px"
                  src="https://registry.nightly.app/email/img/Discord.png"
                  alt=""
                />
              </a>
            </div>
          </div>
        </td>
      </tr>
    </table>
    <table
      width="100%"
      cellpadding="0"
      cellspacing="0"
      style="
        background-color: #6067f9;
        padding: 0 64px;
        max-width: 640px;
        height: 80px;
        text-align: center;
      "
    >
      <tr>
        <td>
          <img
            style="height: 45px"
            src="https://registry.nightly.app/email/img/nc_cloud.png"
            alt=""
          />
        </td>
      </tr>
    </table>
  </body>
</html>
"##;
