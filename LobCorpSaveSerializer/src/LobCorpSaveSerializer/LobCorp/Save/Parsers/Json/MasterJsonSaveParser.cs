using System.Collections.Generic;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public class MasterJsonSaveParser : JsonSaveParserBase
	{
		public MasterJsonSaveParser() : base(SaveType.Master)
		{
		}
		protected override Dictionary<string, object> Parse(JObject save)
		{
			var result = new Dictionary<string, object>();

			result.CopyValue<string>(save, "saveVer");
			result.CopyValue<float>(save, "playTime");
			result.CopyValue<int>(save, "lastDay");
			result.CopyValue<int>(save, "checkPointDay");

			return result;
		}
	}
}
