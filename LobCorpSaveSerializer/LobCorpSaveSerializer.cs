using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization.Formatters.Binary;

namespace LobCorp.Save
{
	public class LobCorpSaveSerializer
	{
		public static LobCorpSave Deserialize(Stream stream)
		{
			BinaryFormatter binaryFormatter = new BinaryFormatter();
			Dictionary<string, object> save = binaryFormatter.Deserialize(stream) as Dictionary<string, object>;
			return new LobCorpSave(save);
		}
	}
}
