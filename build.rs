fn main() {
    // Chỉ kích hoạt khi biên dịch nhắm tới nền tảng Windows
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winres::WindowsResource::new();

        // Manifest tương thích Windows Vista/7/8/8.1/10/11, Per-Monitor V2 DPI & Modern Controls
        res.set_manifest(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity
      version="3.5.3.0"
      processorArchitecture="*"
      name="HenryPlusPlus.MemReduct"
      type="win32"
  />
  <description>Mem Reduct - High-performance memory cleaner</description>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <!-- highestAvailable: Chạy quyền Admin nếu người dùng thuộc nhóm Admin, hoặc quyền thường mà không lỗi -->
        <requestedExecutionLevel level="highestAvailable" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
  <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
    <application>
      <!-- Windows 10 & 11 -->
      <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}" />
      <!-- Windows 8.1 -->
      <supportedOS Id="{1f676c76-80e1-4239-95bb-83d0f6d0da78}" />
      <!-- Windows 8 -->
      <supportedOS Id="{4a2f28e3-53b9-4441-ba9c-d69d4a4a6e38}" />
      <!-- Windows 7 -->
      <supportedOS Id="{35138b9a-5d96-4fbd-8e2d-a2440225f93a}" />
      <!-- Windows Vista -->
      <supportedOS Id="{e2011457-1546-43c5-a5fe-008deee3d3f0}" />
    </application>
  </compatibility>
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2, PerMonitor</dpiAwareness>
    </windowsSettings>
  </application>
  <dependency>
    <dependentAssembly>
      <assemblyIdentity
          type="win32"
          name="Microsoft.Windows.Common-Controls"
          version="6.0.0.0"
          processorArchitecture="*"
          publicKeyToken="6595b64144ccf1df"
          language="*"
      />
    </dependentAssembly>
  </dependency>
</assembly>
"#);

        res.set("FileDescription", "Mem Reduct");
        res.set("ProductName", "Mem Reduct");
        res.set("OriginalFilename", "memreduct.exe");
        res.set("LegalCopyright", "Copyright (c) 2011-2026 Henry++");
        res.set("CompanyName", "Henry++");

        // Nếu có tệp icon memreduct.ico trong thư mục dự án thì nhúng vào
        if std::path::Path::new("memreduct.ico").exists() {
            res.set_icon("memreduct.ico");
        }

        // Biên dịch an toàn, không làm gián đoạn build nếu thiếu công cụ rc
        let _ = res.compile();
    }
}
