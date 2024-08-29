using TauriSharp;
using TauriSharp.Services;
using TauriSharp.Utils;

ProgramArgs programArgs = Initialization.GetArgs(args);

WebApplicationBuilder builder = WebApplication.CreateSlimBuilder(args);
builder.Logging.ClearProviders();
builder.Services.AddGrpc();

WebApplication app = builder.Build();

app.MapGrpcService<GreeterService>();
        
app.Run(programArgs.ToString());