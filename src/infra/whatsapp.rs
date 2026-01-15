use qrcode::QrCode;

/// Renderiza QR code no terminal
pub fn print_qr(payload: &str) {
    println!("\n==================== QR CODE DE AUTENTICAÇÃO ====================\n");
    println!("Escaneie este QR code com o aplicativo do WhatsApp para autenticar.");
    println!("\n---------------------------------------------------------------\n");
    if let Ok(qr) = QrCode::new(payload.as_bytes()) {
        let rendered = qr
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build();

        println!("{}", rendered);
    }
    println!("\n---------------------------------------------------------------\n");
    println!("Payload: {}", payload);
    println!("===============================================================\n");
}
