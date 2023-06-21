module.exports = {
  docs: [
    'home',
    {
      type: 'category',
      label: '💻 Application',
      collapsed: false,
      items: ['application/connect', 'application/send', 'application/sendAll']
    },
    {
      type: 'category',
      label: '📱 Client',
      collapsed: false,
      items: [ 'client/connect', 'client/sign', 'client/signAll', 'client/push']
    },
  ]
}
