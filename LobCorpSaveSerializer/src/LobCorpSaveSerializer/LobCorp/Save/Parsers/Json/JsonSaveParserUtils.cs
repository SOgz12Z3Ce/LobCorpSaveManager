using Newtonsoft.Json.Linq;
using WorkerSprite;

namespace LobCorp.Save.Parsers.Json
{
	public static class JsonSaveParserUtils
	{
		public static WorkerSpriteSaveData.Pair ParseHairStyle(JObject hairData)
		{
			return new WorkerSpriteSaveData.Pair
			{
				a = hairData["a"].Value<long>(),
				b = hairData["b"].Value<int>(),
			};
		}
		public static WorkerSpriteSaveData.ColorData ParseHairColor(JObject hairData)
		{
			return new WorkerSpriteSaveData.ColorData
			{
				r = hairData["r"].Value<float>(),
				g = hairData["g"].Value<float>(),
				b = hairData["b"].Value<float>(),
				a = hairData["a"].Value<float>(),
			};
		}
	}
}
