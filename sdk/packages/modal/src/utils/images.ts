export const svgToBase64 = (svg: string) => {
    const base64 = btoa(svg)

    return 'data:image/svg+xml;base64,' + base64
}